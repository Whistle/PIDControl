extern crate gtk;
extern crate serial;

mod pid_gui;
mod serial_communication;

fn main() {
    pid_gui::show();
}
