#[no_mangle]
pub extern fn lib_test() {
    println!("Hello, world!");
    println!("-All code above me is exextued via the dll-\n");
}