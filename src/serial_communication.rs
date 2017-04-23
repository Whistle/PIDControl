extern crate serial;

use std::time::Duration;
use serial::prelude::*;
use std::io::{BufReader, BufRead};
use std::thread;


const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud115200,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};

pub fn connect() {
    let mut port = serial::open("/dev/ttyACM0");

    if port.is_ok() {
        let thread_handle = thread::spawn(move ||{
            interact(&mut port.unwrap());
        });
    } else {
        println!("Error connecting..");
    }
}

fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::from_secs(10)));

    let mut buf: Vec<u8> = (0..255).collect();

    //try!(port.read(&mut buf[..])
    let mut line = String::new();
    let mut file = BufReader::new(port);
    while true {
        file.read_line(&mut line);
        println!("{}", &mut line);
    }
    Ok(())
}