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

    pub fn debug_read_dtm_reg(&self, reg_addr: u32) -> Result<u32, Error> {
        if reg_addr > 4 {
            panic!("Only register 0 - 4 available!");
            //return Err(Error::AreaMissing{area: "Only register 0 - 4 available!".to_string(),})
        }
        let start_addr = (reg_addr * 4) as usize;
        let mut dst = [0; 1];
        LittleEndian::read_u32_into(&self.debug[start_addr..(start_addr + 4)], &mut dst);
        Ok(dst[0] as u32)
    }

    pub fn debug_write_dtm_reg(&mut self, reg_addr: u32, data: u32) -> Result<(), Error> {
        if reg_addr > 4 {
            panic!("Only register 0 - 4 available!");
            //return Err(Error::AreaMissing{area: "Only register 0 - 4 available!".to_string(),})
        }
        let start_addr = (reg_addr * 4) as usize;
        LittleEndian::write_u32(&mut self.debug[start_addr..(start_addr + 4)], data);
        Ok(())
    }

    pub fn debug_read_dm_reg(&mut self, reg_addr: u32) -> Result<u32, Error> {
        let write_result = self.debug_write_dtm_reg(4, reg_addr);
        assert_eq!(write_result.is_ok(), true);

        let read_result = self.debug_read_dtm_reg(3);
        match read_result {
            Ok(result) => Ok(result),
            Err(err) => panic!("Unable to read DM reg: {}", err),
        }
    }

    pub fn debug_write_dm_reg(&mut self, reg_addr: u32, data: u32) -> Result<(), Error> {
        let write_result = self.debug_write_dtm_reg(4, reg_addr);
        assert_eq!(write_result.is_ok(), true);

        let write_data_result = self.debug_write_dtm_reg(3, data);
        match write_data_result {
            Ok(_) => Ok(()),
            Err(err) => panic!("Unable to write DM reg: {}", err),
        }
    }
}
