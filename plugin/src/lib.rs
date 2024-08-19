#[no_mangle]
extern "C-unwind" fn add() {
    println!("i am in plugin");
    panic!();
}
