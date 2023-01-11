use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the random number!");

    let the_number = rand::thread_rng().gen_range(1..=100);
    // println!("the secret is {the_number} btw");

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {guess}");

        match guess.cmp(&the_number) {
            Ordering::Less => println!("Too big"),
            Ordering::Greater => println!("Too small"),
            Ordering::Equal => {
                println!("yes.");
                break;
            }
        };
    }
}
