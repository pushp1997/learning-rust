use std::{io, cmp::Ordering};
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret Number is: {}", secret_number);

    println!("Please input your guess: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // let guess: u16 = guess.trim().parse().expect("Could not parse user input");

        let guess: u16 = match guess.trim().parse() {
            Err(e) => {
                eprintln!("{} {}", "Could not parse user input:".red(), e);
                continue;
            },
            Ok(num) => num,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too little".yellow()),
            Ordering::Equal => {
                println!("{}", "You guessed it right".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big".yellow()),
        }
    }
}
