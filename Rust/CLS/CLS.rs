fn main(){
    println!("Hello World!");
    // Not sure how but this clears the terminal?
    print!("{esc}c", esc = 27 as char);
}