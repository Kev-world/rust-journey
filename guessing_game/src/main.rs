use rand::Rng;
use std::cmp::Ordering;
use std::io; // Input/Output Library

fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess number: ");

        /*
        The :: syntax in the ::new line indicates that new is an associated function of the String type.
        An associated function is a function that’s implemented on a type, in this case String.
        This new function creates a new, empty string
         */
        let mut guess = String::new();
        /*
        create a variable to store the user input
        let mut apple = 5; -> Mutable
        let apple = 5; -> Immutable
         */

        // Receiving the user input
        io::stdin()
            .read_line(&mut guess) // To handle the input
            .expect("Failed to read line"); // To handle error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
        }
    }
}
