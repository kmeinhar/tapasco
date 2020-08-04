/*
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo
 * (see https://github.com/esa-tu-darmstadt/tapasco).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::allocator::{Allocator, DriverAllocator, GenericAllocator};
use crate::dma::{DMAControl, DirectDMA, DriverDMA};
use crate::job::Job;
use crate::pe::PEId;
use crate::scheduler::Scheduler;
use crate::tlkm::tlkm_access;
use crate::tlkm::tlkm_ioctl_create;
use crate::tlkm::tlkm_ioctl_destroy;
use crate::tlkm::tlkm_ioctl_device_cmd;
use crate::tlkm::DeviceId;
use memmap::MmapMut;
use memmap::MmapOptions;
use prost::Message;
use snafu::ResultExt;
use std::collections::VecDeque;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use libc::c_int;

pub mod status {
    include!(concat!(env!("OUT_DIR"), "/tapasco.status.rs"));
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Device {} unavailable: {}", id, source))]
    DeviceUnavailable {
        source: std::io::Error,
        id: DeviceId,
    },

    #[snafu(display("Memory area {} not found in bitstream.", area))]
    AreaMissing { area: String },

    #[snafu(display("Decoding the status core failed: {}", source))]
    StatusCoreDecoding { source: prost::DecodeError },

    #[snafu(display(
        "Could not acquire desired mode {:?} for device {}: {}",
        access,
        id,
        source
    ))]
    IOCTLCreate {
        source: nix::Error,
        id: DeviceId,
        access: tlkm_access,
    },

    #[snafu(display("PE acquisition requires Exclusive Access mode."))]
    ExclusiveRequired {},

    #[snafu(display("Could not destroy device {}: {}", id, source))]
    IOCTLDestroy { source: nix::Error, id: DeviceId },

    #[snafu(display("Scheduler Error: {}", source))]
    SchedulerError { source: crate::scheduler::Error },

    #[snafu(display("Allocator Error: {}", source))]
    AllocatorError { source: crate::allocator::Error },

    #[snafu(display("Mutex has been poisoned"))]
    MutexError {},

    #[snafu(display("Unknown device type {}.", name))]
    DeviceType { name: String },
}
type Result<T, E = Error> = std::result::Result<T, E>;

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        Error::MutexError {}
    }
}

pub type DeviceAddress = u64;
pub type DeviceSize = u64;

#[derive(Debug, Getters)]
pub struct OffchipMemory {
    #[get = "pub"]
    allocator: Mutex<Box<dyn Allocator + Sync + Send>>,
    #[get = "pub"]
    dma: Box<dyn DMAControl + Sync + Send>,
}

#[derive(Debug)]
pub struct DataTransferLocal {
    pub data: Box<[u8]>,
    pub from_device: bool,
    pub to_device: bool,
    pub free: bool,
}

#[derive(Debug)]
pub struct DataTransferAlloc {
    pub data: Box<[u8]>,
    pub from_device: bool,
    pub to_device: bool,
    pub free: bool,
    pub memory: Arc<OffchipMemory>,
}

#[derive(Debug)]
pub struct DataTransferPrealloc {
    pub data: Box<[u8]>,
    pub device_address: DeviceAddress,
    pub from_device: bool,
    pub to_device: bool,
    pub free: bool,
    pub memory: Arc<OffchipMemory>,
}

#[derive(Debug)]
pub enum PEParameter {
    Single32(u32),
    Single64(u64),
    DeviceAddress(DeviceAddress),
    DataTransferLocal(DataTransferLocal),
    DataTransferAlloc(DataTransferAlloc),
    DataTransferPrealloc(DataTransferPrealloc),
}

#[derive(Debug, Getters)]
pub struct Device {
    #[get = "pub"]
    status: status::Status,
    #[get = "pub"]
    id: DeviceId,
    #[get = "pub"]
    vendor: u32,
    #[get = "pub"]
    product: u32,
    #[get = "pub"]
    name: String,
    access: tlkm_access,
    scheduler: Arc<Scheduler>,
    platform: MmapMut,
    arch: Arc<MmapMut>,
    offchip_memory: Vec<Arc<OffchipMemory>>,
    tlkm_file: Arc<File>,
    tlkm_device_file: Arc<File>,
}

impl Drop for Device {
    fn drop(&mut self) {
        trace!("Device dropped.");
        match self.finish() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
}

impl Device {
    pub fn new(
        tlkm_file: Arc<File>,
        id: DeviceId,
        vendor: u32,
        product: u32,
        name: String,
    ) -> Result<Device> {
        trace!("Mapping status core.");
        let tlkm_dma_file = Arc::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("/dev/tlkm_{:02}", id))
                .context(DeviceUnavailable { id: id })?,
        );

        let s = {
            let mmap = unsafe {
                MmapOptions::new()
                    .len(8192)
                    .offset(0)
                    .map(&tlkm_dma_file)
                    .context(DeviceUnavailable { id: id })?
            };
            trace!("Mapped status core: {}", mmap[0]);

            // copy from device to avoid alignment errors that occur on certain devices
            // e.g. ZynqMP
            let mut mmap_cpy = [0; 8192];
            for i in 0..8192 {
                mmap_cpy[i] = mmap[i];
            }
            //mmap_cpy.clone_from_slice(&mmap[..]);

            status::Status::decode_length_delimited(&mmap_cpy[..]).context(StatusCoreDecoding)?
        };

        trace!("Status core decoded: {:?}", s);

        let platform_size = match &s.platform_base {
            Some(base) => Ok(base.size),
            None => Err(Error::AreaMissing {
                area: "Platform".to_string(),
            }),
        }?;

        // This falls back to PCIe and Zynq allocation using the default 4GB at 0x0
        info!("Using static memory allocation due to lack of dynamic data in the status core.");
        let mut allocator = Vec::new();
        if name == "pcie" {
            info!("Allocating the default of 4GB at 0x0 for a PCIe platform");
            allocator.push(Arc::new(OffchipMemory {
                allocator: Mutex::new(Box::new(
                    GenericAllocator::new(0, 4 * 1024 * 1024 * 1024, 64).context(AllocatorError)?,
                )),
                dma: Box::new(DriverDMA::new(&tlkm_dma_file)),
            }));
        } else if name == "zynq" || name == "zynqmp" {
            info!("Using driver allocation for zynq based platform.");
            allocator.push(Arc::new(OffchipMemory {
                allocator: Mutex::new(Box::new(
                    DriverAllocator::new(&tlkm_dma_file).context(AllocatorError)?,
                )),
                dma: Box::new(DriverDMA::new(&tlkm_dma_file)),
            }));
        } else {
            return Err(Error::DeviceType { name: name });
        }

        let platform = unsafe {
            MmapOptions::new()
                .len(platform_size as usize)
                .offset(8192)
                .map_mut(&tlkm_dma_file)
                .context(DeviceUnavailable { id: id })?
        };

        let arch_size = match &s.arch_base {
            Some(base) => Ok(base.size),
            None => Err(Error::AreaMissing {
                area: "Platform".to_string(),
            }),
        }?;

        let arch = Arc::new(unsafe {
            MmapOptions::new()
                .len(arch_size as usize)
                .offset(4096)
                .map_mut(
                    &OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(format!("/dev/tlkm_{:02}", id))
                        .context(DeviceUnavailable { id: id })?,
                )
                .context(DeviceUnavailable { id: id })?
        });

        let mut pe_local_memories = VecDeque::new();
        for pe in s.pe.iter() {
            match &pe.local_memory {
                Some(l) => {
                    pe_local_memories.push_back(Arc::new(OffchipMemory {
                        allocator: Mutex::new(Box::new(
                            GenericAllocator::new(0, l.size, 1).context(AllocatorError)?,
                        )),
                        dma: Box::new(DirectDMA::new(l.base, l.size, arch.clone())),
                    }));
                }
                None => (),
            }
        }

        let scheduler = Arc::new(
            Scheduler::new(&s.pe, &arch, pe_local_memories, &tlkm_dma_file)
                .context(SchedulerError)?,
        );

        let mut device = Device {
            id: id,
            vendor: vendor,
            product: product,
            access: tlkm_access::TlkmAccessTypes,
            name: name,
            status: s,
            scheduler: scheduler,
            platform: platform,
            arch: arch,
            offchip_memory: allocator,
            tlkm_file: tlkm_file,
            tlkm_device_file: tlkm_dma_file,
        };

        device.create(tlkm_access::TlkmAccessMonitor)?;

        Ok(device)
    }

    fn finish(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn acquire_pe(&self, id: PEId) -> Result<Job> {
        self.check_exclusive_access()?;
        trace!("Trying to acquire PE of type {}.", id);
        let pe = self.scheduler.acquire_pe(id).context(SchedulerError)?;
        trace!("Successfully acquired PE of type {}.", id);
        Ok(Job::new(pe, &self.scheduler))
    }

    fn check_exclusive_access(&self) -> Result<()> {
        if self.access != tlkm_access::TlkmAccessExclusive {
            Err(Error::ExclusiveRequired {})
        } else {
            Ok(())
        }
    }

    pub fn create(&mut self, access: tlkm_access) -> Result<()> {
        if self.access == access {
            trace!(
                "Device {} is already in access mode {:?}.",
                self.id,
                self.access
            );
            return Ok(());
        }

        self.destroy()?;

        let mut request = tlkm_ioctl_device_cmd {
            dev_id: self.id,
            access: access,
        };

        trace!("Device {}: Trying to change mode to {:?}", self.id, access,);

        unsafe {
            tlkm_ioctl_create(self.tlkm_file.as_raw_fd(), &mut request).context(IOCTLCreate {
                access: access,
                id: self.id,
            })?;
        };

        self.access = access;

        if access == tlkm_access::TlkmAccessExclusive {
            trace!("Access changed to exclusive, resetting all interrupts.");
            self.scheduler.reset_interrupts().context(SchedulerError)?;
        }

        trace!("Successfully acquired access.");
        Ok(())
    }

    pub fn destroy(&mut self) -> Result<()> {
        if self.access != tlkm_access::TlkmAccessTypes {
            trace!("Device {}: Removing access mode {:?}", self.id, self.access,);
            let mut request = tlkm_ioctl_device_cmd {
                dev_id: self.id,
                access: self.access,
            };
            unsafe {
                tlkm_ioctl_destroy(self.tlkm_file.as_raw_fd(), &mut request)
                    .context(IOCTLDestroy { id: self.id })?;
            }
            self.access = tlkm_access::TlkmAccessTypes;
        }

        Ok(())
    }

    pub fn design_frequency_mhz(&self) -> Result<f32> {
        let freq = self
            .status
            .clocks
            .iter()
            .find(|&x| x.name == "Design")
            .unwrap_or(&status::Clock {
                name: "".to_string(),
                frequency_mhz: 0,
            })
            .frequency_mhz;
        Ok(freq as f32)
    }

    pub fn debug_offset(&self) -> Result<c_int> {
        let freq = self
            .status
            .platform
            .iter()
            .find(|&x| x.name == "PLATFORM_COMPONENT_DEBUG")
            .unwrap_or(&status::Platform {
                name: "".to_string(),
                offset: 0,
                size: 0,
            })
            .offset;
        Ok(freq as c_int)
    }

    pub fn default_memory(&self) -> Result<Arc<OffchipMemory>> {
        Ok(self.offchip_memory[0].clone())
    }

    pub fn num_pes(&self, pe: PEId) -> usize {
        self.scheduler.num_pes(pe)
    }
}
