extern crate serial;

use std::time::Duration;
use serial::prelude::*;
use std::io::{BufReader, BufRead};
use std::thread;
use std::sync::mpsc::Sender;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud115200,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};

pub fn connect(tx: Sender<String>) {
    let port = serial::open("/dev/ttyACM0");

    if port.is_ok() {
        thread::spawn(move ||{
            interact(&mut port.unwrap(), tx.clone());
        });
    } else {
        tx.send(String::from("Unable to open port"));
    }
}

fn interact(port: &mut SerialPort, tx: Sender<String>) -> serial::Result<()> {
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::from_secs(10)));

    let mut line = String::new();
    let mut file = BufReader::new(port);

    while file.read_line(&mut line).is_ok() {
        print!("{}", &mut line);
        tx.send(String::from("hallo"));
        line.clear();
    }
    
    Ok(())
}