use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("guess a number");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{guess} is not a number");
                continue;
            },
        };

        println!("you picked the {guess} shit");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => {
                println!("Too big!")
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
