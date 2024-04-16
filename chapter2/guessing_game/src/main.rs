use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secretNum: u32 = rand::thread_rng().gen_range(1..=100); // start..=end inclusive of start and end
    println!("the secret number: {secretNum}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin() // if use std:io not used the we write std::io::stdin
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("please type a number between 1 and 100(inclusive)");
    println!("you guessed: {guess}");
    match guess.cmp(&secretNum){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win"),
    }
}  
