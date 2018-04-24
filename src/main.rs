extern crate gtk;
extern crate gdk;

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

    // Load stylesheet
    match gdk::Screen::get_default() {
        Some(screen) => {
            let css_path = match get_resource_path("style.css") {
                Ok(val) => val,
                Err(err) => { panic!("Error getting resource path: {:?}", err); },
            };
            let css_path_str = match css_path.to_str() {
                Some(val) => val,
                None => "",
            };
            let css_provider = gtk::CssProvider::new();
            match css_provider.load_from_path(css_path_str) {
                Ok(_) => {},
                Err(err) => { panic!("Error loading css: {:?}", err); },
            };

            gtk::StyleContext::add_provider_for_screen(&screen, &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        },
        None => {},
    };

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

    // Add endpoint button
    let add_button = gtk::Button::new_with_label("Add First GPG Sync Endpoint");
    match add_button.get_style_context() {
        Some(style_context) => {
            style_context.add_class("add-button");
        },
        None => {}
    };

    // Box layout
    let box_layout = gtk::Box::new(gtk::Orientation::Vertical, 10);
    box_layout.pack_start(&add_button, false, false, 0);
    box_layout.pack_end(&statusbar, false, false, 0);

    // Start the GUI
    window.add(&box_layout);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
