extern crate rand;
use std::io;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {

        println!("Please guess a number from 1-100.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read user input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
