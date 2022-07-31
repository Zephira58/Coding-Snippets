mod lib;
use crate::lib::*;
use std::io;

fn main() {
    lib_test();
    //get user input
    let mut input = String::new();
    println!("Press enter to close");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}