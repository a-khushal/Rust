use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin() // if use std:io not used the we write std::io::stdin
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed: {guess}");
}  
