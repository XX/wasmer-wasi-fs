#![no_main]

#[no_mangle]
pub extern "C" fn start() {
    std::fs::read_dir("/").unwrap();
}
