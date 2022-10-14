use futures::executor::block_on;
use rand::Rng;
use settimeout::set_timeout;
use std::cmp::Ordering;
use std::io;
use std::time::Duration;

async fn game() {
    println!("Guess the passcode to escape the Rusty Dungeon!");
    set_timeout(Duration::from_secs(1)).await;
    println!("You see an old note on the wall. It reads: ");
    set_timeout(Duration::from_secs(1)).await;
    println!("'The passcode is a number between 1 and 100.'");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut lives = 5;

    while lives > 0 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! :(");
                lives -= 1;
            }
            Ordering::Greater => {
                println!("Too big! :(");
                lives -= 1;
            }
            Ordering::Equal => {
                println!("The door unlocks and you escape the dungeon!");
                break;
            }
        }
        println!("You have {lives} guesses remaining...");
        if lives == 0 {
            println!("The secret passcode was {secret_number}");
            println!("You have run out of guesses! You are trapped in the dungeon forever!");
            break;
        }
    }
}
fn main() {
    block_on(game());
}
