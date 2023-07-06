use rand::Rng;
use core::num;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

            let guess: u32 = match guess.trim()
                                        .parse() {
                                              Ok(num) => num,
                                              Err(_) => continue,
                                             };
                              

        println!("You guessed: {guess}");

        println!("The secret number is {}", secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            
            }
        }
    }
}
