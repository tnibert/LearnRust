#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// area as a method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
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

    println!(
        "The area with function of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area with method of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("Square: {:?}", sq);
}

// area as a function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
