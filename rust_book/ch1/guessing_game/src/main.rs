use std::io;

// use rand::prelude::*;

fn main() {

    println!("RNG + my guess!");
    println!("Enter a guess below: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read");

    let secret_number = rand::random_range(1..=100);

    println!("Your guess: {} and the secret number {}", guess, secret_number);

}