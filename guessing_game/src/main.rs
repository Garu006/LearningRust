use std::io; // standard library input/output module
use std::cmp::Ordering; //standard library comparison moule
use rand::Rng; //crate that provides random number generation functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate a random number between 1 and 100

    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); // create a mutable variable guess and initialize it with a empty string

        io::stdin() // get the standard input handle
            .read_line(&mut guess) 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //comparing the guess to the secret number
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
