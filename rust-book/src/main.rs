struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let square = Rectangle::new_square(2);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
}
