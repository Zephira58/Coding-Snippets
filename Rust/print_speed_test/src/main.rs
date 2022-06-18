use colour::*;
use std::{thread, time::Duration};

fn main() {
    let mut n = 0;
    loop{
        green!("Number: ");
        thread::sleep(Duration::from_millis(5));
        red!("{:?}\n", &n);
        n = n + 1;
    }
}
