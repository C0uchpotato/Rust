use std::io;
use std::rand;

fn main() {
    println!("Guess the number");
    
    println!("Input your guess");

    let input = io::stdin().read_line()
                            .ok()
                             .expect("Failed to read line");
    println!("You guessed: {}", input);
}
