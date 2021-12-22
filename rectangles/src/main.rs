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

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 200,
    };

    let rect2 = Rectangle {
        width: 200,
        height: 300,
    };

    let rect3 = Rectangle::square(400);

    println!("The rect1 is {:#?}", rect1);
    println!("The rect2 is {:#?}", rect2);
    println!("The rect3 is {:#?}", rect3);

    println!("The area of the rect1 is {}", rect1.area());
    println!("The area of the rect2 is {}", rect2.area());

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1 ? {}", rect2.can_hold(&rect1));
}