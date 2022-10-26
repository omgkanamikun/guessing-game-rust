use std::cmp::Ordering;
use std::io;

use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number was generated");

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("error during parsing input");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        };

        println!("{} {guess}", "you guessed".bright_yellow());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".blue()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "you win!".green());
                break;
            }
        }
    }
}
