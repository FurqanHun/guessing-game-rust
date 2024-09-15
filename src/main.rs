use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number breh (1..100)!");
    let secret_number = random_number();

    loop {
        println!("Select a number (1..100): ");
        let guess = user_input();
        println!("You guessed: {}", guess);

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

fn random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn user_input() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            user_input()
        }
    };
    guess
}
