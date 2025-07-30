use std::io;
use rand::Rng;

fn main() {
    println!("Let's play a guessing game!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("What's my number between 1 and 100?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
