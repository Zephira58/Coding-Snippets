use dotenv;
use std::process::Command;

pub fn cls(){
    print!("{esc}c", esc = 27 as char);
}

pub fn credits(){

    println!("Made by @Xanthus58 (on github) In my spare time");
    println!("Check out my other works at https://github.com/Xanthus58");
    println!("Email me on 'Xanthus58@protonmail.com'");
    println!("Feel free to fork; submit issues; or otherwise interact with the project!");

    let mut line = String::new();
    println!("Press Enter to return");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
}

use dotenv;
fn readEnv(envID: String) {
    let envValue = dotenv::var(envID).unwrap();
    return envValue;
}

fn getUserInt() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input.trim().parse().expect("Not a valid number")
}

use std::process::Command;
fn terminalInterface(mainArg: String, subArg: String) {
    Command::new("ls")
        .args([mainArg, subArg])
        .spawn()
        .expect("Failed to execute terminalInterface process");
}

fn getUserString() -> String {
    let mut line = String::new();//Assigns a variable to a new empty string
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();//Reads the user inputted string and writes it to the line variable
    return line;
}