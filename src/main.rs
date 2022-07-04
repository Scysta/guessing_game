// Input/output library from the standard library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is {}", secret_number);

    // loop is an infinite loop, similar to while(True)
    loop {
        println!("Please input your guess.");

        // Creates a variable with let; makes it mutable 
        // with mut; creates an empty string with String::new()
        // (this is from std)
        let mut guess = String::new();

        // Calling stdin to proccess user input
        // It's important to note that these three lines;
        // stdin, read_line and expect belong to the same 
        // line of code, that's why the semicolon only
        // goes at the end; read_line and expect are methods
        // of stdin().
        // Therefore we can write all that on one line
        // io::stdin().read_line(&mut guess).expect("...");
        io::stdin()
            // Receiving input from the user and stores it
            // in the mutable variable "guess"; read_line
            // appends to a string, without overwriting it
            // The & before mut indicates that we are accessing
            // a "reference" of guess, and because of references
            // being also immutable we need to specify &mut 
            // before the variable instead of &guess
            .read_line(&mut guess)
            // This handles exceptions/errors
            // expect returns a value io::Result, which is a 
            // common value in a lot of Rust libraries.
            // Results variants are Ok and Err; their 
            // purposes are intuitive
            .expect("Failed to read line");
            // Ok this is actually sick: if read_line inserts
            // input on the variable, but it also return a 
            // io::Result type, which has an expect method
            // and that's why we can chain these 3, they're
            // all methods of the previous one

        // What's interesting about this line is that the curly
        // brackets {} act as a python format string, so wathever 
        // is in the args list after the string will be formatted
        // inside it and printed correctly in order: the first
        // pair of brackets will correspond to the first argument,
        // the second to the first argument and so on
        // This line makes it so we can reuse the guess variable and
        // assign it another value. In this case, the method trim 
        // is equivalent to python's split and the method parse
        // parses a string into a number of type we declare in 
        // guess: «u32». Similarly to before, expect handles an error
        // if the user would input something that cannot be parsed to
        // a number type (expect is changed to a match statement
        // so we can ignore a non-number input)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); 

        // This is basically a series of ifs but using a match
        // statement that compares a value with every "arm" it's
        // inside it. In this case the method "cmp" return a value
        // Ordering::{Less,Greater,Equal} and see if it matches
        // with any of its three arms
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
