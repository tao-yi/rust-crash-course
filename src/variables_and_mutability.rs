use std::mem;

pub fn run() {
    let a: i32;
    a = 15;
    println!("{}", a);

    let mut x = 5;
    println!("before {}", x);
    x = 6;
    println!("after {}", x);

    types();
}

fn types() {
    // integer type
    let x: i8 = -1;
    print!("i8 max {}, min {}", i8::MAX, i8::MIN);
    let x: u8 = 8;
    print!("u8 max {}, min {}", i8::MAX, i8::MIN);
    let x: i16 = -1;
    print!("i16 max {}, min {}", i16::MAX, i16::MIN);
    let x: u16 = 1;
    print!("u16 max {}, min {}", u16::MAX, u16::MIN);
    let y: i32 = -1;
    let y: u32 = y as u32;
    println!("{} {}", y, y == u32::MAX);

    // isize and usize depend on the architecture of the computer
    // 64bits on 64-bit arch
    // 32btis on 32-bit arch
    let a: isize = 1; // 8 bytes
    let b: usize = 1; // 8 bytes
    println!("{} {}", mem::size_of_val(&a), mem::size_of_val(&b));

    // by default is i32
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("{} {} {} {} {}", decimal, hex, octal, binary, byte);

    let x = 2.0; // by default is f64
    let f: f32 = x;

    let b = false;

    let c = 'z'; // 4 bytes unicode code point
    let c = '😁';
    println!("{} {}", mem::size_of::<char>(), mem::size_of_val(&c));

    // tuple
    let tup: (i32, f64, (u8, bool)) = (500, 6.4, (1, false));
    println!("{:?}", tup);

    let tup: (&str, [i32; 2]) = ("hello", [1, 2]);
    println!("{:?}", tup);

    let arr: [(i32, bool); 2] = [(1, false), (2, true)];
    println!("{:?}", arr);

    let arr = ["hello"; 5];
    println!("{:?}", arr);

    // initialize 2-D array [[0,0],[0,0]]
    let _2d_arr = [[0; 2]; 2];
    println!("{:#?}", _2d_arr);
}
