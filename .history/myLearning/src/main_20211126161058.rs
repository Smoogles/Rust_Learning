use std::io;

pub fn main() {
    println!("Hello, world!");
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("expected a string!");

    println!("you guessed wrong");
}
