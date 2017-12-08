// use extern crate
extern crate rand;

// use a standard prelude library
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!"); // prompt "guess the number"

    let secret_number = rand::thread_rng().gen_range(1, 101); // note to myself : je comprends toujours pas cette notation

   // println!("The secret number is: {}", secret_number);

    loop {
      let mut guess = String::new(); 

      // call a handle from the io library and 2 methods, the second one is the panic! to handle the error
      io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

     // let guess: u32 = guess.trim() .parse()
      //  .expect("Please type a number!");

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => {
          println!("You win!");
          break;
        }
    }

  }
}
