use std::mem;

// a custom type
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn run() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?} {:?}", home, loopback);
    enum_data();
    enum_method();
    options();
    match_flow();
    bind_enum_to_values();
    enum_option();
    if_let();
}

fn enum_data() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn enum_method() {
    struct Content {
        prefix: String,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        Body(Content),
    }

    impl Message {
        fn call(&self) {
            println!("size: {}", mem::size_of_val(self));
        }
    }

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("Hello World!"));
    let m4 = Message::Body(Content {
        prefix: String::from("hello"),
    });

    let messages = [m1, m2, m3, m4];
    for m in messages {
        m.call();
    }
}

fn options() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;
    println!(
        "{} {} {}",
        some_number.unwrap(),       // return the contained value
        some_string.unwrap(),       // caution: this method will panic if `None`
        absent_number.unwrap_or(5)  // return the contained value or a default one
    );
}

fn match_flow() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        let c = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
        return c;
    }

    println!("{}", value_in_cents(Coin::Penny));
}

fn bind_enum_to_values() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from:{:?}", state);
                25
            }
        }
    }

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn enum_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn if_let() {
    let config_max = Some(3_u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
