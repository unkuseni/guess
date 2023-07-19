use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// Main function to run the guessing game.
fn main() {
    println!("Pick a number!");

    // Generate a random secret number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // Start an infinite loop for the game.
    loop {
        println!("Please input your guess.");

        // Read the user's input.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user's input into an unsigned 32-bit integer.
        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("You guessed: {}", guess);

        // Compare the user's guess with the secret number and give feedback.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");
            break;
        },
        }
    }
}