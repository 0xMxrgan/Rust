use std :: io;
use std :: cmp :: Ordering;
use rand :: Rng;

fn main() {

    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    loop {

    println!("Please input your guess:");

    let mut guess = String::new();

    io :: stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

     let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("I will accept only a number as an answer!");
        continue;
        }
     };                                                                         // Checking if it's a number

    println!("You guessed {}.", guess);

    match guess.cmp(&secret_number) {
        Ordering :: Less => println!("Too small!"),
        Ordering :: Equal => {
            println!("Congratulations!\nYou win.");
            break;
        }
        Ordering :: Greater => println!("Too big!"),
    }
    }
}
