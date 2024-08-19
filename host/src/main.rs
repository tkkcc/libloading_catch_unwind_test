fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    unsafe {
        let lib = libloading::Library::new("target/debug/libplugin.so")?;
        let func: libloading::Symbol<extern "C-unwind" fn()> = lib.get(b"add")?;
        std::panic::catch_unwind(|| {
            func();
        });
    };
    Ok(())
}
