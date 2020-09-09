/*
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo
 * (see https:///github.com/esa-tu-darmstadt/tapasco).
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
 * along with this program. If not, see <http:///www.gnu.org/licenses/>.
 */

use byteorder::{ByteOrder, LittleEndian};
use std::fs::OpenOptions;
use std::sync::Arc;
use snafu::ResultExt;
use memmap::MmapMut;
use memmap::MmapOptions;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Device unavailable: {}", source))]
    DeviceUnavailable { source: std::io::Error },

    #[snafu(display("Register not found: {}", area))]
    AreaMissing { area: String },

    #[snafu(display("Debug Error: {}", error))]
    DebugError{ error: String },
}

const DATA0_REG: u32 = 0x4;
const DATA1_REG: u32 = 0x5;
const DMCONTROL_REG: u32 = 0x10;
const DMSTATUS_REG: u32 = 0x11;
const COMMAND_REG: u32 = 0x17;

const DMCONTROL_DMACTIVE: u32 = 1 << 0;
const DMCONTROL_RESUMREQ: u32 = 1 << 30;
const DMCONTROL_HALTREQ: u32 = 1 << 31;

const DMSTATUS_ALLHALTED: u32 = 1 << 9;

const COMMAND_WRITE: u32 = 1 << 16;
const COMMAND_TRANSFER: u32 = 1 << 17;
const COMMAND_AARSIZE_32BIT: u32 = 2 << 20;
const COMMAND_CMDTYPE_REG: u32 = 0 << 24;
const COMMAND_CMDTYPE_MEM: u32 = 2 << 24;

pub struct JTAGDebug {
    debug: MmapMut,
}

impl JTAGDebug {
    pub fn new() -> Result<JTAGDebug, Error> {
        trace!("Mapping RV debug area.");
        let tlkm_debug_file = Arc::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tlkm_debug_00")
                .context(DeviceUnavailable {})?,
        );

        let debug = unsafe {
            MmapOptions::new()
                .len(20)
                .offset(0)
                .map_mut(&tlkm_debug_file)
                .context(DeviceUnavailable {})?
        };
        trace!("Mapped RV debug area: {}", debug[4]);


        Ok(JTAGDebug {
            debug: debug,
        })
    }

    pub fn read_dtm_reg(&self, reg_addr: u32) -> Result<u32, Error> {
        if reg_addr > 4 {
            panic!("Only register 0 - 4 available!");
        }
        let start_addr = (reg_addr * 4) as usize;
        let mut dst = [0; 1];
        LittleEndian::read_u32_into(&self.debug[start_addr..(start_addr + 4)], &mut dst);
        Ok(dst[0] as u32)
    }

    pub fn write_dtm_reg(&mut self, reg_addr: u32, data: u32) -> Result<(), Error> {
        if reg_addr > 4 {
            panic!("Only register 0 - 4 available!");
        }
        let start_addr = (reg_addr * 4) as usize;
        LittleEndian::write_u32(&mut self.debug[start_addr..(start_addr + 4)], data);
        Ok(())
    }

    pub fn read_dm_reg(&mut self, reg_addr: u32) -> Result<u32, Error> {
        let write_result = self.write_dtm_reg(4, reg_addr);
        assert_eq!(write_result.is_ok(), true);

        let read_result = self.read_dtm_reg(3);
        match read_result {
            Ok(result) => Ok(result),
            Err(err) => panic!("Unable to read DM reg: {}", err),
        }
    }

    pub fn write_dm_reg(&mut self, reg_addr: u32, data: u32) -> Result<(), Error> {
        let write_result = self.write_dtm_reg(4, reg_addr);
        assert_eq!(write_result.is_ok(), true);

        let write_data_result = self.write_dtm_reg(3, data);
        match write_data_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to write DM reg: {}", err),
        }
    }

    pub fn activate_dm(&mut self) -> Result<(), Error> {
        // In dmcontrol set activate bit
        let write_result = self.write_dm_reg(DMCONTROL_REG, DMCONTROL_DMACTIVE);
        match write_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to activate DM: {}", err),
        }
    }

    pub fn halt_core(&mut self) -> Result<(), Error> {
        // In dmcontrol set halt and activate bit
        let write_result = self.write_dm_reg(DMCONTROL_REG, DMCONTROL_HALTREQ | DMCONTROL_DMACTIVE);
        match write_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to halt core: {}", err),
        }
    }

    pub fn resume_core(&mut self) -> Result<(), Error> {
        // In dmcontrol set resume and activate bit
        let write_result = self.write_dm_reg(DMCONTROL_REG, DMCONTROL_RESUMREQ | DMCONTROL_DMACTIVE);
        match write_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to resume core: {}", err),
        }
    }

    pub fn is_core_halted(&mut self) -> Result<bool, Error> {
        let read_result = self.read_dm_reg(DMSTATUS_REG);
        match read_result {
            Ok(value) => Ok((value | DMSTATUS_ALLHALTED) != 0),
            Err(err) => panic!("Unable to activate DM: {}", err),
        }
    }

    /**
     * Read the given register of the RISC-V core with abstract command
     */
    pub fn abstract_register_read(&mut self, reg_addr: u32) -> Result<u32, Error> {
        let command = reg_addr      // Address in lowest bits
            | COMMAND_TRANSFER
            | COMMAND_AARSIZE_32BIT
            | COMMAND_CMDTYPE_REG;
        // Set the command register
        let write_result = self.write_dm_reg(COMMAND_REG, command);
        assert_eq!(write_result.is_ok(), true);

        let read_result = self.read_dm_reg(DATA0_REG);
        match read_result {
            Ok(value) => Ok(value),
            Err(err) => panic!("Unable to activate DM: {}", err),
        }
    }

    pub fn abstract_memory_read(&mut self, mem_addr: u32) -> Result<u32, Error> {
        let write_addr_result = self.write_dm_reg(DATA1_REG, mem_addr);
        assert_eq!(write_addr_result.is_ok(), true);

        let command = COMMAND_AARSIZE_32BIT
            | COMMAND_CMDTYPE_MEM;
        // Set the command register
        let write_result = self.write_dm_reg(COMMAND_REG, command);
        assert_eq!(write_result.is_ok(), true);

        let read_result = self.read_dm_reg(DATA0_REG);
        match read_result {
            Ok(value) => Ok(value),
            Err(err) => panic!("Unable to activate DM: {}", err),
        }
    }

    pub fn abstract_memory_write(&mut self, reg_addr: u32, data: u32) -> Result<(), Error> {
        let write_addr_result = self.write_dm_reg(DATA1_REG, reg_addr);
        assert_eq!(write_addr_result.is_ok(), true);
        let write_data_result = self.write_dm_reg(DATA0_REG, data);
        assert_eq!(write_data_result.is_ok(), true);

        let command = COMMAND_WRITE
            | COMMAND_AARSIZE_32BIT
            | COMMAND_CMDTYPE_MEM;
        // Set the command register
        let write_command_result = self.write_dm_reg(COMMAND_REG, command);
        match write_command_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to activate DM: {}", err),
        }
    }

}
