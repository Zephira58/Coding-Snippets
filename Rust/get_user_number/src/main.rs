fn get_user_input() -> u32 {
    let mut input = String::new();
    println!("\n-Please enter a number between 1 and 100-");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input.trim().parse().expect("Not a valid number")
}