// *
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // `::` => associated function
        // new is the associated function to the type String
        // best called a `static method`
        let mut guess = String::new();

        // call stdin is the associated function of io
        // `&` is a reference to access one piece of data
        // * Without declaring std::io we would have to write this line as std::io::stdin
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        // `{}` placeholder for let variables
        // you can use as many as you want in respect to length of variables used
        println!("You guessed: {}", guess);

        // Switching from an `expect` call to a `match` expression
        // is how you prevent crashing on error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("Please type a number!");

        // The `guess` variable, from user input,
        // is compared to `secret_number` variable, random integer.
        // The `match` expression returns a result to be matched to the enums in `cmp`
        match guess.cmp(&secret_number) {
            // Assumptions
            Ordering::Less => println!("Too small!"), //Enum -1
            Ordering::Greater => println!("Too big!"), // Enum 1
            Ordering::Equal => {
                println!("You win!"); // Enum 0
                break;
            }
        }
    }
}
