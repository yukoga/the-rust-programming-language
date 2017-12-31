extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

//    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input the number which you guess it correct one.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The number you guessed is : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small you guessed!"),
            Ordering::Greater => println!("Too big you guessed!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
