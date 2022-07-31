#![crate_type = "cdylib"]

//TODO: Figure out how tf i'm supposed to make reading the DLL dynamic instead of static

// Program made by Xanths58
// Check out my other works at https://xanthus58.github.io/Xanthus58/
// or https://github.com/Xanthus58

mod lib;
use crate::lib::*;
use std::io;

fn main() {
    //clear screen
    print!("\x1B[2J\x1B[1;1H");
    credits();

    println!("-Introduction-");
    println!("Hello, Welcome to a simple DLL library test.");
    println!("Included in the DLL is a simple program that calculates all the different math variations of 2 numbers.");
    println!("While the program handles gathering user inputs and displaying the results to the screen.\n");
    
    //get user input as a number
    println!("Enter a number: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f32 = num1.trim().parse().expect("Please type a number!");

    println!("Enter another number: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f32 = num2.trim().parse().expect("Please type a number!");

    let math = math(num1, num2);
    println!("{}", math);

    println!("\nPress enter to exit.");
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
}
