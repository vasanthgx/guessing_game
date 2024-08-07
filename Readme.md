

### Code Explanation

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Standard Library (`std`)

#### `std::io` Module
The `std::io` module provides functionalities for input and output operations. In this code, `io::stdin().read_line(&mut guess)` reads a line of input from the standard input (keyboard) and stores it in the `guess` variable.

#### `std::cmp` Module and `Ordering` Enum
The `std::cmp` module provides functionality for comparisons. The `Ordering` enum has three variants: `Less`, `Greater`, and `Equal`. These are used to represent the result of a comparison, such as when comparing the user's guess to the secret number.

### `rand` Crate and `Rng` Trait
The `rand` crate is an external crate used for generating random numbers. The `Rng` trait defines methods that random number generators implement. In this code, `rand::thread_rng().gen_range(1..=100)` generates a random number between 1 and 100.

### Rust Concepts

#### `loop` Keyword
The `loop` keyword creates an infinite loop. The loop continues until it encounters a `break` statement.

#### `match` Statement
The `match` statement is used for pattern matching. It takes a value and matches it against a series of patterns, executing the corresponding block of code for the first matching pattern. In this code, `match guess.cmp(&secret_number)` compares the user's guess to the secret number and prints messages based on whether the guess is too small, too big, or correct.

#### `enum`
An `enum` (short for "enumeration") is a type that can be any one of several variants. The `Ordering` enum in this code has three variants: `Less`, `Greater`, and `Equal`. Enums are useful for defining a type that can take on a discrete set of possible values.

### `expect()` Method
The `expect()` method is used to handle the `Result` type. It either returns the value if the `Result` is `Ok`, or panics with the provided error message if the `Result` is `Err`. In this code, `io::stdin().read_line(&mut guess).expect("Failed to read line")` will panic with "Failed to read line" if reading from standard input fails.

### Putting It All Together

1. **Imports**: The code begins by importing necessary modules and traits.
    ```rust
    use std::io;
    use rand::Rng;
    use std::cmp::Ordering;
    ```

2. **Main Function**: The main function is the entry point of the program.
    ```rust
    fn main() {
    ```

3. **Print Message**: Print a welcome message to the user.
    ```rust
    println!("Guess the number!");
    ```

4. **Generate Secret Number**: Generate a random number between 1 and 100.
    ```rust
    let secret_number = rand::thread_rng().gen_range(1..=100);
    ```

5. **Prompt User for Input**: Print a message asking the user to input their guess.
    ```rust
    println!("Please input your guess.");
    ```

6. **Loop for Guessing**: Start an infinite loop to allow the user to guess repeatedly.
    ```rust
    loop {
    ```

7. **Read User Input**: Read the user's guess from standard input.
    ```rust
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    ```

8. **Parse User Input**: Attempt to parse the user's guess as a `u32`. If parsing fails, continue the loop.
    ```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    ```

9. **Print User's Guess**: Print the user's guess.
    ```rust
    println!("You guessed: {guess}");
    ```

10. **Compare Guess to Secret Number**: Use a `match` statement to compare the guess to the secret number and print appropriate messages.
    ```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    ```

