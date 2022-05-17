use std::{thread, time::Duration};

fn main(){
    print!("\x1B[2J");
    let mut line = String::new();
    println!("Enter your name:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello, {}", line);
    thread::sleep(Duration::from_secs(5));
    
}