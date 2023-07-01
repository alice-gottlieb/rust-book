use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Should have read line from console.");

    println!("You guessed: {guess}");

    match guess.trim().parse::<u32>() {
        Ok(num) => {
            if num == secret_number {
                println!("You win!");
            } else {
                println!("You lose!, The secret number was: {secret_number}");
            }
        },
        Err(_) => println!("You didn't enter a number."),
    }
}
