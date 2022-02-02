use std::io;

use rand::random;

pub fn main() {
    println!("Hello, world!");
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("expected a string!");


    let the_random = random();
    if let the_random == guess {
        println!("Correct");
    }
    if the_random != guess {
        printlnl!("Incorrect");
         

    }


}
