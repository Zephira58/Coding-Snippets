use std::{thread, time::Duration};

fn main(){
    loop{
    print!("{esc}c", esc = 27 as char);
    let mut line = String::new();
    
    println!("Will you help the man? (Y/N)");
    
    std::io::stdin().read_line(&mut line).unwrap();
    
    let Y = "Y";
    let y = "y";
    let N = "N";
    let n = "n";
    
    let line = line.trim();
    print!("{esc}c", esc = 27 as char);
    
    
    if line == y || line == Y{
        println!("The man gives you a peice of gold and thanks you.");
        thread::sleep(Duration::from_secs(5));
        break
    } 
    else if line == n || line == N{
        println!("You walk past the man and ignore him");
        thread::sleep(Duration::from_secs(5));
        break
    }
    else{
        println!("Invalid input, please respond with 'Y' or 'N'");
        thread::sleep(Duration::from_secs(5));
    }
    }  
 }