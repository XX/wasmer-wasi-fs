#[no_mangle]
pub fn start() {
    std::fs::read_dir("/").unwrap();
}
