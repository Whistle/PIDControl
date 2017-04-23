extern crate gtk;
extern crate serial;


use std::thread;

mod pid_gui;
mod serial_communication;

fn main() {
    pid_gui::show();
}
