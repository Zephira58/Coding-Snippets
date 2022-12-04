fn getUserString() -> String {
    let mut line = String::new();//Assigns a variable to a new empty string
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();//Reads the user inputted string and writes it to the line variable
    return line;
}