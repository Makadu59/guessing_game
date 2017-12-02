// use a standard prelude library
use std::io;

fn main() {
    println!("Guess the number!"); // prompt "guess the number"

    println!("Please input your guess."); // prompt "Please input your guess"

    let mut guess = String::new(); // initialize a variable named guess

// call a handle from the io library and 2 methods, the second one is the panic! to handle the error
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
