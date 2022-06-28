use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // Add values
    marks.insert("Rust programing", 96);
    marks.insert("Python programing", 92);
    marks.insert("Java programing", 88);
    marks.insert("C++ programing", 84);

    // Find length of hashmap
    println!("How many subjects I have studied: {}", marks.len());

    // Get a single value
    match marks.get("Rust programing") {
        Some(mark) => println!("Mark for Rust programming: {}", mark),
        _ => println!("No mark for Rust programming"),
    }

    // Remove a value
    marks.remove("Java programing");

    // loop through hashmap
    for (key, value) in &marks {
        println!("For {} you got {}", key, value);
    }

    //Check for value
    println!("Did you study C++? {}", marks.contains_key("Python"));
}
