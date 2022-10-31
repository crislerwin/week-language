use std::io;
use rand::Rng;


pub(crate) fn run() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    print!("The secret number is: {secret_number} ");
    print!("Please input your guess.");
    
   let mut guess = String::new();
   io::stdin() .read_line(&mut guess)
   .expect("Failed to read line");

println!("You guessed: {guess}");
    
}

