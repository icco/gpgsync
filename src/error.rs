use std;

#[derive(Debug)]
pub enum ErrorKind {
    CurrentExeNotFound(std::io::Error),
    FilesystemError(std::io::Error),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind: kind }
    }
}
