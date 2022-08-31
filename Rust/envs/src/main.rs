use dotenv;

fn main() {
    println!("{}", dotenv::var("X").unwrap());
}