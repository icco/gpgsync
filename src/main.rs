extern crate gtk;

use gtk::prelude::*;

mod error;
mod common;

use common::{log, get_resource_path, get_version_string};

fn main() {
    let version_string = match get_version_string() {
        Ok(val) => val,
        Err(err) => { panic!("Error getting version string: {:?}", err); },
    };

    log("main", &format!("starting GPG Sync {}", &version_string));

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    //window.set_border_width(0);
    window.set_default_size(400, 200);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Header bar
    let logo_path = match get_resource_path("gpgsync.png") {
        Ok(val) => val,
        Err(err) => { panic!("Error getting resource path: {:?}", err); },
    };
    let logo = gtk::Image::new_from_file(logo_path);
    let header_bar = gtk::HeaderBar::new();
    header_bar.pack_start(&logo);
    header_bar.set_title("GPG Sync");
    window.set_titlebar(&header_bar);

    // Status bar
    let version_label = gtk::Label::new(Some(version_string.as_str()));
    let statusbar = gtk::Statusbar::new();
    let statusbar_box = match statusbar.get_message_area() {
        Some(val) => val,
        None => { panic!("Error getting statusbar message area"); },
    };
    statusbar_box.pack_end(&version_label, false, false, 0);

    // Box layout
    let box_layout = gtk::Box::new(gtk::Orientation::Vertical, 10);
    box_layout.pack_end(&statusbar, false, false, 0);

    // Start the GUI
    window.add(&box_layout);
    window.show_all();

    gtk::main();
}
