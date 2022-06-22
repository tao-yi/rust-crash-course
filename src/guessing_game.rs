// By default, Rust has a few items defined in the standard library that it brings into the scope of every program.
use std::io;
// the Rng traits defines methods that random number generatos implement
// and this trait must be in scope for us to use those methods
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
