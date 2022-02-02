use std::io;

pub fn main() {
    println!("Hello, world!");
    let mut guess = std::String;
    println!("Guess a number");
    io::stdin()
    .read_line(&guess)
    .expect("expected a string!");

}
