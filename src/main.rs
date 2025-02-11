use std::io;
use rand::Rng;
use std::cmp::Ordering;

// to run an .rs file you must change the directory 
// to the said where the said file is placed

// rustc main.rs
// .\main.exe

// cargo run

fn main() {
    println!("Guess the number!\ndasdasdadsa");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}