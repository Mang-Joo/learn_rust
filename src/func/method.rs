#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        dbg!(self.width) > dbg!(other.width) && dbg!(self.height) > dbg!(other.height)
    }

    pub fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} sqaure pixels.",
        rect.area()
    );

    let other_rect = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "Can rect hold other? {}", rect.can_hold(&other_rect)
    );

    let square = Rectangle::square(10);

    println!("This is square {square:#?}");
}