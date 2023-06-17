use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1) generate random secret number
    let secret = rand::thread_rng().gen_range(1..101);

    // 2) Ask user for a guess
    println!("guess the number (hint {}):", secret);

    loop {
        // 3) Read user input as String
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 4) Verify user number and tell User
        let guess: i32 = guess.trim().parse().expect("Failed, please type a number!");
        println!("You guessed: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("\nThanks for playing!");
    println!("The secret number was: {}", secret);
}
