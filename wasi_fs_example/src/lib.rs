use std::{fs, io};

#[no_mangle]
pub fn start() {
    fs::read_dir("/")
        .unwrap()
        .map(|res| res.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
}
