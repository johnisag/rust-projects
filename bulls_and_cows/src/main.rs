use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Bull and Cows");

    // Generate a random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..11);

    // capture attempts
    let mut attempts = 0;

    // loop for multiple attempts
    loop{
        println!("Please input Number:");

        // Create a mutable variable to store the user's guess
        let mut guess = String::new();
    
        // Read the user's guess from the standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user's guess to a number
        let guess: u32 = match guess.trim().parse() {	
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };

        // increase attempts
        attempts += 1;

        if guess < 1 || guess > 10 {
            println!("The secret number is between 1 and 10");
            continue;
        }

        match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if attempts == 10 {
            println!("You have tried 10 times, game over");
            break;
        }
    }
}

