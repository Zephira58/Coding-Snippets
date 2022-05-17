use std::{thread, time::Duration};

fn main() {
    println!("Hello World!");
    thread::sleep(Duration::from_millis(5000));
    println!("This is my third attempt at learning the basics of rust; this time i'm learning how to sleep a thread.");
    thread::sleep(Duration::from_secs(5))
}