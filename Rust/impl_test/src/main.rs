struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {}x{}", self.width, self.height);
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 5,
        height: 5,
    };
    my_rectangle.print_description();
    println!("Is this rectangle a square: {}", my_rectangle.is_square());
}

