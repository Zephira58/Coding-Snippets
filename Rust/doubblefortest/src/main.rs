fn main() {
    // Outer loop to iterate from 1 to 10
    for i in 1..=10 {
        // Inner loop to print the current number
        for _ in 1..=i {
            print!("{} ", i); // Print the current number
        }
        println!(); // Move to the next line after each iteration of the outer loop
    }
}