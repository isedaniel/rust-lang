use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100);
    println!("Random number is {number}");

    loop {
        println!("Enter your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Line could not be read");

        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        println!("You guessed: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You won!");
                break;
            },
        }
    }
}
