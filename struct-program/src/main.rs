#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    
    dbg!(&rect1);

    if rect1.width() {
        println!(
            "The area of the rectangle is {} square pixels", 
            rect1.area()
        );
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
