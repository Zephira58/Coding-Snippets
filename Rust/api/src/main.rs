//https://docs.rs/reqwest/latest/reqwest/blocking/index.html

use std::error::Error;
use open;

//fn post() {
//    let client = reqwest::blocking::Client::new();
//    let res = client.post("http://httpbin.org/post")
//    .body("the exact body that is sent")
//    .send();
//}
//
//fn get() {
//    let resp = reqwest::blocking::get("https://inspirobot.me/api?generate=true")?.text()?; //gets the response from the api
//    println!("{:#?}", resp);
//    open::that(resp)?;
//}

//fn main() -> Result<(), Box<dyn Error>> {
//    let mut amount = String::new();
//    println!("How many quotes do you want to see?");
//    let _b1 = std::io::stdin().read_line(&mut amount).unwrap();
//    let amount: u32 = amount.trim().parse()?;
//    
//    Ok(for x in 0..amount{
//    let resp = reqwest::blocking::get("https://inspirobot.me/api?generate=true")?.text()?; //gets the response from the api
//    println!("{:#?}", resp);
//    open::that(resp)?;
//
//    })
//}

fn main() {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

println!("body = {:?}", body);
}