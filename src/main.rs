extern crate gtk;
extern crate serial;

use gtk::prelude::*;
use gtk::{ScrolledWindow, TextView, Label, Orientation, Box, Button, Window, WindowType, Entry};
use serial_communication::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;

const WINDOW_WIDTH: i32 = 850;
const WINDOW_HEIGHT: i32 = 350;
const ALIGN_RIGHT: f32 = 1f32;

mod serial_communication;

fn main() {
   if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let main_window = Window::new(WindowType::Toplevel);
    main_window.set_title("PID Control v0.42");
    main_window.set_default_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let btn_update = Button::new_with_label("Update Controller");
    let btn_connect = Button::new_with_label("Connect");
    btn_connect.set_size_request(100, 30);

    let pid_values_box = Box::new(Orientation::Horizontal, 6);
    let h_main_box = Box::new(Orientation::Vertical, 6);
    h_main_box.add(&pid_values_box);
    h_main_box.add(&btn_connect);

    pid_values_box.add(&btn_update);
    
    // P - Entry and Label
    let p_value = Entry::new();
    let p_label = Label::new_with_mnemonic(Some("P:"));
    p_value.set_width_chars(5);
    p_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&p_label);
    pid_values_box.add(&p_value);

    // I - Entry and Label
    let i_value = Entry::new();
    let i_label = Label::new_with_mnemonic(Some("I:"));
    i_value.set_width_chars(5);
    i_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&i_label);
    pid_values_box.add(&i_value);
    
    // D - Entry and Label
    let d_value = Entry::new();
    let d_label = Label::new_with_mnemonic(Some("D:"));
    d_value.set_width_chars(5);
    d_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&d_label);
    pid_values_box.add(&d_value);

    // TextView
    let debug_scrolled = ScrolledWindow::new(None, None);
    debug_scrolled.set_hexpand(true);
    debug_scrolled.set_vexpand(true);
    let debug_view = TextView::new();
    debug_view.set_editable(false);
    debug_view.set_cursor_visible(false);
    debug_scrolled.add(&debug_view);
    h_main_box.add(&debug_scrolled);

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(move || {
        loop {
            println!("{}", rx.recv().unwrap());
        }

    });

    // show window
    main_window.add(&h_main_box);
    main_window.show_all();

    // adding events
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    btn_update.connect_clicked(|_| {
        println!("refreshing pid values...");
    });

    btn_connect.connect_clicked(move |_| {
        connect(tx.clone());
    });

    gtk::main();  
}
