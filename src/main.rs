use std::io;  //includes io (input/output) library which is part of standard library
use std::cmp::Ordering;  // includes compare and less, greater, and equal
use rand::Rng;  // we added rand = "0.8.5" to Cargo.toml first, this line puts rng trait in scope

fn main() {
    println!("Guess the number!");  //macro (!) that prints to screen

    let secret_number = rand::thread_rng().gen_range(1..=100); //generates random number between 1 and 100, uses rand documentation you can find with "cargo doc --open"

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  //"let" creates variable, "mut" indicated mutability/changability, sets it equal to new empty instance of String

        io::stdin()  // calls standard input function from io which we included above
            .read_line(&mut guess)
            .expect("Failed to read line");  // if read_line result is Err, program crashes and displays the text, if it is Ok, expect returns the value the user entered

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };  //match function tells it to return the number if ok, or if any err. restart at the top of the loop

        println!("You guessed: {guess}");  //curly brackets allow the variable within the string

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  //exits it out of the loop
            }
        }
    }
}
