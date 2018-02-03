#![feature(try_trait)]
#![feature(iterator_try_fold)]

extern crate byteorder;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate sane;

use sane::*;
use sane::status::Status;
use sane::error::Error;

use std::io::prelude::*;
use std::net::TcpStream;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

fn main() {
    pretty_env_logger::init();

    let mut stream = TcpStream::connect("192.168.1.20:6566").expect("Failed to connect");
    stream.set_nodelay(true);

    init(&mut stream);

    let devices = request_device_list(&mut stream).unwrap();

    let device = devices
        .iter()
        .inspect(|device| {
            info!(
                "{} - {} - {} - {}",
                device.name, device.vendor, device.model, device.kind
            )
        })
        .take(1)
        .next()
        .unwrap();

    let handle = match open_device(&device, &mut stream) {
        Ok(result) => match result {
            OpenResult::Handle(handle) => {
                println!("Received handle {}", handle);
                Some(handle)
            }
            OpenResult::AuthRequired(resource) => {
                println!("Received authentication resource {}", resource);
                None
            }
        },
        Err(e) => {
            error!("{:?}", e);
            None
        }
    };

    let options = match get_option_descriptors(handle.unwrap(), &mut stream) {
        Ok(options) => options,
        Err(e) => {
            error!("{:?}", e);
            panic!();
        }
    };

    println!("Closing device {}", &device.name);
    close_device(handle.unwrap(), &mut stream);
}
