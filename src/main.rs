use std::io as stdio;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::from("You guessed: ");

    stdio::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{guess}");
}
