#[no_mangle]
pub extern fn lib_test() {
    println!("Hello, world!");
    println!("-All code above me is exextued via the dll-\n");
}

pub fn math(num1: f32, num2: f32) {
    println!("\n-Math-");
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);
    println!("{} / {} = {}", num1, num2, num1 / num2);
}