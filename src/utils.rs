use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader};
use gdk;
use gtk;
use gtk::prelude::*;

use error::{Error, ErrorKind};

pub fn log(location: &str, msg: &str) {
    // TODO: Only log in debug mode
    println!("[{}] {}", location, msg);
}

pub fn get_resource_path(filename: &str) -> Result<PathBuf, Error> {
    log("common", &format!("get_resource_path {}", filename));

    // TODO: Right now, we're assuming GPG Sync is not packaged and always run from the source tree
    // using `cargo run`. Therefore, resources are in ../../share/.
    let mut pathbuf = match env::current_exe() {
        Ok(v) => v,
        Err(e) => { return Err(Error::new(ErrorKind::CurrentExeNotFound(e))) },
    };
    pathbuf.pop();
    pathbuf.pop();
    pathbuf.pop();
    pathbuf.push("share");
    pathbuf.push(filename);
    Ok(pathbuf)
}

pub fn get_version_string() -> Result<String, Error> {
    log("common", "get_version_string");

    let version_path = match get_resource_path("version.txt") {
        Ok(val) => val,
        Err(err) => { return Err(err) },
    };

    // Read the version from disk
    let file = match File::open(version_path.as_path()) {
        Ok(val) => val,
        Err(err) => { return Err(Error::new(ErrorKind::FilesystemError(err))) },
    };
    let mut contents = String::new();
    let mut reader = BufReader::new(file);
    match reader.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(err) => { return Err(Error::new(ErrorKind::FilesystemError(err))) },
    };

    Ok(contents.trim().to_string())
}

pub fn load_stylesheet() {
    log("utils", "load_stylesheet");

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
}

// Add a CSS class to a GTK widget
pub fn add_class<T: gtk::WidgetExt>(widget: &T, class_name: &str) {
    match widget.get_style_context() {
        Some(style_context) => {
            style_context.add_class(class_name);
        },
        None => {}
    };
}

// Remove a CSS class to a GTK widget
pub fn remove_class<T: gtk::WidgetExt>(widget: &T, class_name: &str) {
    match widget.get_style_context() {
        Some(style_context) => {
            style_context.remove_class(class_name);
        },
        None => {}
    };
}
