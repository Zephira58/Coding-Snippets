use std::{thread, time::Duration};

fn main(){
    let mut line = String::new();

    println!("Will you help the man? (Y/N)");

    std::io::stdin().read_line(&mut line).unwrap();

    let y = "Y";
    let line = line.trim();
    print!("{esc}c", esc = 27 as char);
    
    if line == y{
        println!("The man gives you a peice of gold and thanks you.");
        thread::sleep(Duration::from_secs(5))
    } 
    else{
        println!("You walk past the man and ignore him");
        thread::sleep(Duration::from_secs(5))
    }
 }