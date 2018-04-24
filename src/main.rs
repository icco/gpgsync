extern crate gtk;
extern crate gdk;
extern crate gio;

use gtk::prelude::*;

mod error;
mod utils;

use utils::{log, get_resource_path};

fn main() {
    let version_string = utils::get_version_string().unwrap();
    log("main", &format!("starting GPG Sync {}", &version_string));

    // Initialize the GTK app
    let _app = gtk::Application::new("org.firstlookmedia.gpgsync", gio::ApplicationFlags::empty()).unwrap();

    // Load stylesheet
    utils::load_stylesheet();

    // Create window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("GPG Sync");
    window.set_border_width(10);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Logo
    let logo_path = get_resource_path("gpgsync.png").unwrap();
    let logo = gtk::Image::new_from_file(logo_path);

    // Status bar
    let version_label = gtk::Label::new(Some(version_string.as_str()));
    let statusbar = gtk::Statusbar::new();
    let statusbar_box = statusbar.get_message_area().unwrap();
    statusbar_box.pack_end(&version_label, false, false, 0);

    // Add endpoint button
    let add_button = gtk::Button::new_with_label("Add First GPG Sync Endpoint");
    utils::add_class(&add_button, "add-button");

    // Box layout
    let box_layout = gtk::Box::new(gtk::Orientation::Vertical, 10);
    box_layout.pack_start(&logo, false, false, 0);
    box_layout.pack_start(&add_button, false, false, 0);
    box_layout.pack_end(&statusbar, false, false, 0);

    // Start the GUI
    window.add(&box_layout);
    window.show_all();

    gtk::main();
}
