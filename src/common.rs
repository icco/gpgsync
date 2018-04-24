use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader};

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
