use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm = std::fs::read("target/wasm32-wasi/debug/wasi_fs_example.wasm")?;

    let store = Store::default();
    let module = Module::new(&store, wasm)?;

    let mut wasi_env = WasiState::new("wasi_fs_example")
        .preopen_dir("target")?
        .finalize()?;

    let import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;

    let start = instance.exports.get_function("start")?;
    start.call(&[])?;

    Ok(())
}