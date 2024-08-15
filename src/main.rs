// Importing the required modules from the Rust standard library and the rand crate
use std::io; // For input/output operations
use rand::prelude::*; // For random number generation

fn main() {
    // Array of fruit names that the user can guess from
    let guess_list = ["banana", "apple", "mango", "orange", "grapes"];

    // Start an infinite loop to keep asking the user for guesses until they get it right
    loop {
        // Create a random number generator
        let mut rng = thread_rng();
        
        // Generate a random index within the range of the guess_list array
        let index = rng.gen_range(0..guess_list.len());
        
        // Select a random fruit from the guess_list using the generated index
        let random_fruit = guess_list[index];

        // Print the randomly selected fruit (for debugging purposes, or it could be hidden)
        println!("Random Fruit: {}", random_fruit);
        println!("Please enter your guess fruit:");

        // Create a mutable string to store user input
        let mut input = String::new();

        // Read user input from standard input (the terminal)
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the input to remove any extra whitespace and convert it to lowercase
                let fruit_name = input.trim().to_lowercase();
                println!("Selected Fruit: {}", fruit_name);

                // Check if the input is a valid fruit from the guess_list
                if !guess_list.contains(&fruit_name.as_str()) {
                    println!("Invalid fruit! Please enter a valid fruit.");
                    // Continue the loop and prompt the user to guess again
                    continue;
                }

                // Check if the user's guess matches the randomly selected fruit
                match guess_fruit(&fruit_name, &random_fruit) {
                    true => {
                        println!("Correct Guess! You are a winner!");
                        // Exit the loop if the guess is correct
                        break;
                    }
                    false => {
                        // Notify the user to try again if the guess was incorrect
                        println!("Retry");
                    }
                }
            }
            Err(error) => {
                // Handle any potential errors during input reading
                eprintln!("Error: {}", error);
            }
        }
    }
}

// Function to compare the user's guess with the actual random fruit
fn guess_fruit(guess: &str, actual: &str) -> bool {
    // Return true if the guess matches the actual fruit, otherwise false
    guess == actual
}
