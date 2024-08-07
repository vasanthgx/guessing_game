use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is : {}", secret_number);

    println!("Please input your guess.");

   loop { let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("You win!");
                 break;
                     }
    }
}
}


// /////////////////////////Processing a Guess /////////////////////////

// The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.

// Handling potential failure with Result enum, which is a type that represents either success or failure, and is returned by the read_line() function.
//If we want to handle failure in a program, we can use the Result enum. We’ll use the expect method to convert the error into a string.

////////////////Generating a Secret Number /////////////////////////

//The next part of the guessing game program will generate a secret number. We’ll use the rand crate to generate a random number.

//updating main() function with the secret number 

/* 
let secret_number = rand::thread_rng().gen_range(1..=100)
printLn!(The secret number is {}", secret_number)

*/


//////////////////////Comparing the Guess to the Secret Number /////////////////////////

/*
we add another use statement, bringing a type called std::cmp::Ordering into scope from the standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),

We get an error of mismatched types. To overcome this

We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. "Shadowing" lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.

updating the main() function with the following line

let guess: u32 = guess.trim().parse().expect("Please type a number!");
*/


/////////////////////// Allowing Multiple Guesses with Loopin //////////////////

/*

Allowing Multiple Guesses with Looping

The loop keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:

*/

/////////////////////////Quitting after a correct guess /////////////////////////
/*
    Adding the break line after You win! makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of main.
*/

/////////////////////////////Handling Invaild Input////////////////////////
/*


*/