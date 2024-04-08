use std::io; // Include the standard input/outuput library
use std::cmp::Ordering; // Include compare and Ordering
use rand::Rng; // Include random library

fn main() {
    
    println!("Welcome to the magic Guessing Game!\nI have already choosed a number, please enter your guess and let's play.");

    let secret_number = rand::thread_rng().gen_range(1..=10); // Choose the secret number from 1 to 10
    // println!("{}", secret_number);

    loop {
        let mut guess = String :: new(); // Declare variable as mutable
        io::stdin() // This block of lines could have been written in a single one because they are part of a single logical operation
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You've guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try a bigger number!"),
            Ordering::Greater => println!("Try a smaller number!"),
            Ordering::Equal => {
                println!("Great!\nYou win!");
                break;
            }
        }
    }
}
