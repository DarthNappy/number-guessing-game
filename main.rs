use std::io; // Bring in the input/output library from the Rust standard library
use rand::Rng; // Bring in the Rng methods from the rand crate
use std::cmp::Ordering; // Bring in the Ordering enum from the cmp module

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Assign variable 'secret_number' a random number between 1:100

    //println!("The secret number is: {secret_number}");

    loop { // Create a loop for User guessing

        println!("Please input your guess.");

        // Create a mutable variable called guess
        // Assign the variable guess a new empty string
        let mut guess = String::new(); 
    
        io::stdin() // Call the std function from the input/output (io) library
            .read_line(&mut guess) // Call the read line method to get the input from the User and assign it to 'mut guess'
            .expect("Failed to read line"); // Set error handling message

        let guess: u32 = match guess // Annotate 'guess' to String & trim & parse 'guess' so std::cmp can compare to secret_number
            .trim()
            .parse() { // Allow the guessing game to continue if User enters an invalid input
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // Uses the std::cmp method to compare 'guess' with 'secret_number'
            Ordering::Less => println!("Your guess is less than the Secret Number."),
            Ordering::Greater => println!("Your guess is greater than the Secret Number."),
            Ordering::Equal => {
                println!("Awesome! You guessed the Secret Number!");
                break; // End guessing game if User guess is correct
            }
        }
    }
}
