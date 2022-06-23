#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self: &Self
    // this method borrows the Self instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn grow(&mut self, scale: u32) {
        self.width += scale;
        self.height += scale;
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associated functions that aren't methods are often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.grow(10);

    println!(
        "The area of the reactangle is {} square pixels.",
        rect1.area()
    );
    println!("{}", rect1.width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("can hold {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(10);
    println!("{:?}", rect3);
}
