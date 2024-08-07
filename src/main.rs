use std::io;

fn main() {

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}


// /////////////////////////Processing a Guess /////////////////////////

//The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.

// Handling potential failure with Result enum, which is a type that represents either success or failure, and is returned by the read_line() function.
//If we want to handle failure in a program, we can use the Result enum. We’ll use the expect method to convert the error into a string.

////////////////Generating a Secret Number /////////////////////////

//The next part of the guessing game program will generate a secret number. We’ll use the rand crate to generate a random number.