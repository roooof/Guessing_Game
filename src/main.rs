use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\n\n################");
    println!("# Guessin game #");
    println!("################\n\n");
    Game();
}

fn Game() {
    let secret_number = rand::thread_rng().gen_range(1..9);

    loop {
        println!("Guess the number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => break,
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