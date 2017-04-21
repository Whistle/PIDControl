extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Orientation, Box, Button, Window, WindowType, Entry};

const WINDOW_WIDTH = 350;
const WINDOW_HEIGHT = 70;
const ALIGN_RIGHT: f32 = 1f32;

pub fn show() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let main_window = Window::new(WindowType::Toplevel);
    main_window.set_title("PID Control v0.42");
    main_window.set_default_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let button = Button::new_with_label("Update Controller");

    let pid_values_box = Box::new(Orientation::Horizontal, 6);
    let h_main_box = Box::new(Orientation::Vertical, 6);
    h_main_box.add(&pid_values_box);
    h_main_box.add(&button);
    
    // P - Entry and Label
    let p_value = Entry::new();
    let p_label = Label::new_with_mnemonic(Some("P:"));
    p_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&p_label);
    pid_values_box.add(&p_value);

    // I - Entry and Label
    let i_value = Entry::new();
    let i_label = Label::new_with_mnemonic(Some("I:"));
    i_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&i_label);
    pid_values_box.add(&i_value);
    
    // D - Entry and Label
    let d_value = Entry::new();
    let d_label = Label::new_with_mnemonic(Some("D:"));
    d_value.set_alignment(ALIGN_RIGHT);

    pid_values_box.add(&d_label);
    pid_values_box.add(&d_value);
    

    // show window
    main_window.add(&h_main_box);
    main_window.show_all();

    // adding events
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("refreshing pid values...");
    });

    gtk::main(); 
}