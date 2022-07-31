#[no_mangle]
pub fn math(num1: f32, num2: f32) -> String {
    println!("\n-Math-");
    let add = num1 + num2;
    let sub = num1 - num2;
    let mul = num1 * num2;
    let div = num1 / num2;
    let modulo = num1 % num2;
    return format!(
        "{} + {} = {}\n{} - {} = {}\n{} * {} = {}\n{} / {} = {}\n{} % {} = {}",
        num1, num2, add, num1, num2, sub, num1, num2, mul, num1, num2, div, num1, num2, modulo
    );
}
