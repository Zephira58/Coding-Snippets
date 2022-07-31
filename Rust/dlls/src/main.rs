#![crate_type = "dylib"]

mod lib;
use crate::lib::*;
use std::io;

fn main() {
    lib_test();
    //get user input as a number
    println!("Enter a number: ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f32 = num1.trim().parse().expect("Please type a number!");
    println!("Enter another number: ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f32 = num2.trim().parse().expect("Please type a number!");
    math(num1, num2);
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
}