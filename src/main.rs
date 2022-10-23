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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guessed {}", guess);

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
