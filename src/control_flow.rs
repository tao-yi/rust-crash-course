use std::array;

pub fn run() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);

    loops();
    nested_loop();
    loop_return_val();
    while_loop();
    for_in();
}

fn loops() {
    let mut x = 5;
    loop {
        println!("{}", x);
        x -= 1;
        if x == 0 {
            break;
        }
    }
}

fn nested_loop() {
    let mut count = 0;
    // the outer label loop has label 'counting_up
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;
        loop {
            println!("remaining={}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn loop_return_val() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break and return value
        }
    };
    println!("{}", result)
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_in() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {}", element);
    }

    for (i, &num) in a.iter().enumerate() {
        println!("a[{}]={}", i, num);
    }

    let r = (1..4); // Range<i32>
    println!("{}", r.len());

    // (1..4) returns a `Range<i32`, it generates [1,2,3]
    for number in (1..4).rev() {
        println!("{}!", number)
    }
}
