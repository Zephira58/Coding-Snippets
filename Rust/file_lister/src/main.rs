use std::{thread,fs, time::Duration, io::Write};
// use chrono::Utc;
// use colour::*;


fn cls(){
    print!("{esc}c", esc = 27 as char);
}

fn credits(){
    print!("{esc}c", esc = 27 as char);
    println!("Made by Xanthus");
    println!("Check out my other works at https://github.com/Xanthus58");
    println!("Email me at 'Xanthus58@protonmail.com'");
    println!("You can see more information on my website https://xanthus58.github.io/Xanthus58/");
    
    let mut input = String::new();
    println!("Press Enter To Return");
    std::io::stdin().read_line(&mut input).unwrap();
    let minputod_yn = input.trim();
}

fn main() {
    loop{
        cls();
        let paths = fs::read_dir("./").unwrap();
        let mut file = "";
        for file_name in paths {
            let file_name = file_name;
            println!("Name: {}", file_name.unwrap().path().display());
        }
        thread::sleep(Duration::from_secs(5));
        break
    }
}
