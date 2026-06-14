std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); //variable mutable, String::new() creates a new empty string

    io::stdin()
        .read_line(&mut guess) //&mut guess is a mutable reference to the variable guess
        .expect("Failed to read line"); //expect() is a method that returns the value of the result if its ok or not

    println!("Your guess: {guess}");

}
