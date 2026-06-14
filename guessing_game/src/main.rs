use std::io; // standard library input/output module

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable guess and initialize it with a empty string

    io::stdin() // get the standard input handle
        .read_line(&mut guess) 
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
