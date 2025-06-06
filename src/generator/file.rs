use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub fn create_file(path: &str) -> Result<File, std::io::Error> {
    File::create(path)
}

pub fn write_file(path: &str, text: &str) -> Result<(), std::io::Error> {
    match OpenOptions::new().append(true).open(path) {
        Ok(mut file) => file.write_all(text.as_bytes()),
        Err(e) => Err(e),
    }
}
