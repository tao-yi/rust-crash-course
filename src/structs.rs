struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn area(obj: Object) -> u32 {
    obj.width * obj.height
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run() {
    let w = 32;
    let h = 64;
    let o = Object::new(w, h);
    println!("{} {}", w, h);

    let mut user1 = User {
        active: false,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        username: String::from("someusername123"),
    };
    user1.active = true;
    println!("{:#?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername123"),
        ..user1
    };
    println!("{:#?}", user2);
    println!("{:#?}", user1);

    tuple_structs();
    unit_like_structs();
    structs_example();
}

pub fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black({},{},{})", black.0, black.1, black.2);
    println!("origin({},{},{})", origin.0, origin.1, origin.2);
}

pub fn unit_like_structs() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn structs_example() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (width1, height1);

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    println!("The area of the rectangle is {}", area(rect1));

    // a better version
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    fn area1(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("The area of the rectangle is {}", area1(&rect1));
    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);
}
