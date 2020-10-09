extern crate capnp;
extern crate capnpc;
extern crate tapasco;
extern crate snafu;
#[macro_use]
extern crate getset;

mod tapasco_riscv_capnp {
    include!("../schema/tapasco_riscv_capnp.rs");
}

use std::process;
use std::slice;
use std::io::{Read, BufRead, BufReader, BufWriter};
use std::os::unix::net::{UnixStream,UnixListener};
use std::path::{Path, PathBuf};
use crossbeam::thread;
use tapasco::tlkm::*;
use tapasco::job::*;
use tapasco::device::*;
use tapasco::debug::*;
use tapasco::debug::Error as DebugError;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use memmap::MmapMut;
use std::sync::Arc;
use core::fmt::Debug;
use volatile::Volatile;

use capnp::*;

use tapasco_riscv_capnp::*;
use crate::request::request::Which::*;


#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to initialize TLKM object: {}", source))]
    TLKMInit { source: tapasco::tlkm::Error },

    #[snafu(display("Failed to decode TLKM device: {}", source))]
    DeviceInit { source: tapasco::device::Error },

    #[snafu(display("Error while executing Job: {}", source))]
    JobError { source: tapasco::job::Error },

    #[snafu(display("Error while enabling debug: {}", source))]
    DebugEnableError { source: tapasco::job::Error },

    #[snafu(display("Error while creating io Socket: {}", source))]
    DebugIoError { source: std::io::Error },

    #[snafu(display("Error: No applicable PE found"))]
    DebugPeError { },

    #[snafu(display("Error: Got a bad request with value: {}", value))]
    InputError { value: usize},
}
 
const PICCOLO32_PE_ID: usize = 1745;
const SOCKET_FILE_NAME: &str = "riscv-debug.sock";

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Getters)]
pub struct RiscvDebugGenerator {}


impl DebugGenerator for RiscvDebugGenerator {
    fn new(
        &self,
        arch_memory: &Arc<MmapMut>,
        name: String,
        offset: DeviceAddress,
        _size: DeviceSize,
    ) -> Result<Box<dyn DebugControl + Send + Sync>, DebugError> {
        Ok(Box::new(RiscvDebug {
            name: name,
            debug_mem: arch_memory.clone(),
            offset: offset,
        }))
    }
}

#[derive(Debug, Getters)]
pub struct RiscvDebug {
    name: String,
    debug_mem: Arc<MmapMut>,
    offset: DeviceAddress,
}

impl RiscvDebug {
    fn handle_ctrl_c(&self) {
        ctrlc::set_handler(move || {
            println!("received Ctrl+C!");
            match std::fs::remove_file(SOCKET_FILE_NAME) {
                Ok(_) => (),
                Err(e) => panic!("Unable to delete socket file {}: {}", SOCKET_FILE_NAME, e),
            }
            process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");
    }

    fn handle_client(&self) -> Result<()> {
        let listener = UnixListener::bind(SOCKET_FILE_NAME).context(DebugIoError)?;

        self.handle_ctrl_c();

        match listener.accept() {
            Ok((mut stream, addr)) => {
                //stream.write_all(b"Thanks");

                println!("Handling RISC-V debug connection");

                //let mut stream = stream.context(DebugIoError)?;

                loop {

                    // Read a message from socket
                    let mut buffer = [0; 100];
                    match stream.read(&mut buffer).context(DebugIoError)? {
                        x if x < 1 => return Err(Error::InputError {value: x}),
                        x => (),//println!("Got input of size: {}", x),
                    };



                    // Decode the message from socket
                    let mut buffer2 = &buffer[..];
                    let message_reader = serialize::read_message_from_flat_slice(&mut buffer2,
                                            ::capnp::message::ReaderOptions::new()).unwrap();

                    let request = message_reader.get_root::<request::Reader>().unwrap();

                    match request.get_request().which().unwrap() {
                        Read(Ok(read_req)) => {
                            println!("Got read at address: {}", read_req.get_addr());

                            let offset = self.offset as isize;
                            let r = match read_req.get_addr() {
                                // TODO Workaround since DTMCS currently returns wrong val
                                x if x == 8 => 0x71,
                                x => unsafe {
                                    let ptr = self.debug_mem.as_ptr().offset(
                                        offset + (read_req.get_addr() as isize));
                                    let volatile_ptr = ptr as *mut Volatile<u32>;
                                    (*volatile_ptr).read()
                                }
                            };

                            // Respond with a message
                            let mut message = ::capnp::message::Builder::new_default();
                            let mut read_req = message.init_root::<read_rsp::Builder>();

                            read_req.set_data(r as u32);

                            println!("Responding read with: {:#X}", r);

                            let copy = stream.try_clone().expect("try_clone failed");
                            serialize::write_message(copy, &message);

                        },
                        Write(Ok(write_req)) => {
                            println!("Got write at address: {} with {:#X}",
                                     write_req.get_addr(), write_req.get_data());

                            let offset = self.offset as isize;
                            unsafe {
                                let ptr = self.debug_mem.as_ptr().offset(
                                    offset + (write_req.get_addr() as isize));
                                let volatile_ptr = ptr as *mut Volatile<u32>;
                                (*volatile_ptr).write(write_req.get_data())
                            }
                        },
                        _ => panic!("Could not decode request"),
                    }; // reques.get_request().which()
                } // loop
            }, // Ok((mut stream, addr))
            Err(e) => panic!("Error"),
        } // listener.accept()
        Ok(())
    }
}


impl DebugControl for RiscvDebug {
    fn enable_debug(&mut self) -> Result<(), DebugError> {
        println!("Listening to connection in socket: {}", SOCKET_FILE_NAME);

        match self.handle_client() {
            Ok(_) => Ok(()),
            Err(e) => match e {
                Error::DebugIoError{source} => match source.kind() {
                    std::io::ErrorKind::ConnectionReset => {
                        println!("Connection was reset by client, closing debugger!");
                        Ok(())
                    }
                    other_err => {
                        println!("Unknown error while handling client: {}", source);
                        Ok(())
                    }
                }
                other_err => {
                    println!("Unknown error while handling client: {}", other_err);
                    Ok(())
                }
            }
        }
    }
}

fn print_version(tlkm: &TLKM) -> Result<()> {
    let ver = tlkm.version().context(TLKMInit {})?;
    println!("TLKM version is {}", ver);
    Ok(())
}


fn allocate_devices(tlkm: &TLKM) -> Result<Vec<Device>> {
    let mut debug_generators: HashMap<String, Box<dyn DebugGenerator + Sync + Send>> = HashMap::new();
    let debug_generator = RiscvDebugGenerator{};
    debug_generators.insert("DmiDebug".to_string(), Box::new(debug_generator));

    let mut devices = tlkm.device_enum(&debug_generators).context(TLKMInit {})?;

    for x in devices.iter_mut() {
        println!(
            "Device {}: Name: {}, Vendor: {}, Product {}",
            x.id(),
            x.name(),
            x.vendor(),
            x.product()
        );
        println!("Allocating ID {} exclusively.", x.id());
        x.change_access(tapasco::tlkm::tlkm_access::TlkmAccessExclusive)
            .context(DeviceInit {})?;
    }

    Ok(devices)
}

fn print_status(devices: &Vec<Device>) -> Result<()> {
    for x in devices {
        println!("Device {}", x.id());
        println!("{:?}", x.status());
    }
    Ok(())
}

fn enable_debug(devices: &Vec<Device>) -> Result<Job> {
    for x in devices {
        let mut pe1 = x.acquire_pe(PICCOLO32_PE_ID).context(DeviceInit)?;
        pe1.enable_debug().context(DebugEnableError)?;
        return Ok(pe1);
    }
    DebugPeError{}.fail()
}

fn run_test() -> Result<()> {
    let tlkm = TLKM::new().context(TLKMInit {})?;
    print_version(&tlkm)?;
    let devices = allocate_devices(&tlkm)?;
    print_status(&devices)?;
    enable_debug(&devices)?;

    /*
    thread::scope(|s| {
        s.spawn(|_| {
            handle_client(self.debug_mem.clone(), self.offset);
        });
    });
    */
 
    match std::fs::remove_file(SOCKET_FILE_NAME) {
        Ok(_) => (),
        Err(e) => panic!("Unable to delete socket file {}: {}", SOCKET_FILE_NAME, e),
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_test() {
        println!("Application error:\n{}", e);
        process::exit(1);
    }

    println!("Test Finished");
}

