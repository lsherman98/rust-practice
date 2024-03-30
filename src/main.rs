use core::num;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    // println!("correct: {correct}");

    loop {
        println!("Guess a number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again");
                continue;
            }
        };

        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed the correct number");
                break;
            }
        };
    }
}
