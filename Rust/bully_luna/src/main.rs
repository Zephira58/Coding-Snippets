use std::{thread, time::Duration}; // Importing sleep method

//Manually making a class
struct Luna;
impl Luna {
 fn bully(&self) {
   println!("Haha you're a nerd");
 }
 fn cool(&self) {
  println!("Okay maybe your cool");
 }
}

// making a cls function as theres none by default ;-;
fn cls(){
  print!("{esc}c", esc = 27 as char);
}

fn main() {
  let Luna = Luna;
  // gets input from you and asks if your studding
  let mut yn = String::new();
  println!("-Are you studding? (Y/N)-");
  std::io::stdin().read_line(&mut yn).unwrap();
  let yn = yn.trim();

if yn == "y" || yn == "Y"{ // if you say yes it bullies you
  cls();
  Luna.bully();
  thread::sleep(Duration::from_secs(5));
}
else{ // if you say no your cool
  cls();
  Luna.cool();
  thread::sleep(Duration::from_secs(5));
}
}