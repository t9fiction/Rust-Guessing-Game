// Import necessary libraries
use rand::Rng;             // For generating a random number
use std::cmp::Ordering;    // For comparing the guess with the secret number
use std::io;               // For handling input/output operations

fn main() {
    // Print welcome message
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);  // Corrected typo: "secret_nubmer" -> "secret_number"

    // Start an infinite loop for guessing
    loop {
        println!("Please input your guess.");

        // Create a mutable variable to store the user's guess as a string
        let mut guess = String::new();

        // Read the guess from standard input and handle any potential errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess string to a u32 integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing succeeds, use the parsed number
            Err(_) => {
                println!("Please enter a valid number"); // Prompt user if input is invalid
                continue; // Continue the loop without checking further
            }
        };

        // Compare the guess with the secret number and print appropriate response
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   // Guess is smaller than the secret number
            Ordering::Greater => println!("Too big!"),  // Guess is larger than the secret number
            Ordering::Equal => {                        // Guess is correct
                println!("Congratulations! You guessed the correct number!");
                break;                                  // Exit the loop
            }
        }
    }
}
