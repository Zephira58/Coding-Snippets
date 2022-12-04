fn getUserInt() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input.trim().parse().expect("Not a valid number")
}