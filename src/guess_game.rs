use rand::Rng;
use std::{cmp::Ordering, io};
pub fn guess_game() {
    println!("Guess the number");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please guess a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too less"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
