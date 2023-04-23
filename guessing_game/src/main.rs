use std::io;
use rand::Rng;

fn main() {
    println!("Put some shit in here");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("the secret number is {secret_number}");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("could not read line");
    
    println!("you picked the {guess} shit")
}
