fn input(){
    print!("{esc}c", esc = 27 as char); ;//Clears the screen
    let mut line = String::new();//Assigns a variable to a new empty string 
    println!("Enter your name:");//Prompts the user for input
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();//Reads the user inputted string and writes it to the line variable
    println!("Hello, {}", line);//Prints the result
}