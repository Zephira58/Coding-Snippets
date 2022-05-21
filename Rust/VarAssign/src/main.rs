// before asking reddit https://pastebin.com/SJ3D6mk8

use std::{thread, time::Duration};

fn main() {
    print!("{esc}c", esc = 27 as char);
    let mut mod_yn = String::new();
    println!("Are there any modifiers (Y/N)");
    std::io::stdin().read_line(&mut mod_yn).unwrap();
    let mod_yn = mod_yn.trim();

    let mut modifer: f32 = 0.0;
    let mut mod_reason = String::new();
    let mut mod_value = 0;
    if mod_yn == "y" || mod_yn == "Y" {
        mod_value = 1;
        print!("{esc}c", esc = 27 as char);

        let mut input = String::new();
        //I'm attempting to get the input from the user to define a modifer reason and value
        println!("Please enter the modifer: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        modifer = input.trim().parse().expect("Not a valid number");

        println!("Whats the modifer reason: ");
        std::io::stdin().read_line(&mut mod_reason).unwrap();
        mod_reason = mod_reason.trim().to_string();
    }
    //Then print those results to the screen outside of nesting
    print!("{esc}c", esc = 27 as char);
    println!("-Modifer-");
    println!("{}", modifer);
    println!("-Modifer Reason-");
    println!("{}", mod_reason);
    if mod_value == 1 {
        println!("Test value {}", mod_value);
    }
    thread::sleep(Duration::from_secs(5));
}
