use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {number}");

    println!("Enter your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Line could not be read");

    println!("You guessed: {guess}");
}
