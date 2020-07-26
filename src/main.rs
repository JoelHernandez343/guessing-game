use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

/// This is a Guessing game implementation.
/// ## Instructions
/// All instructions are in [the official documentation](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
/// ## Coder
/// Joel H. Check [my Github](https://github.com/JoelHernandez343)
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Secret number: {}", secret_number);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Errors");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

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
