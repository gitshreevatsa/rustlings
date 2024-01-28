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
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 10,
    };

    let sq : Rectangle = Rectangle::square(3);

    println!("{}", sq.area());
    println!("The area of rectangle is: {}", rect1.area());

    println!("Can rect1 hold rect2? {} ", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// #[warn(dead_code)]
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
