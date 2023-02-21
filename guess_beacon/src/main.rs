use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the beacon!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");
        
        // Rust has **SHADOWING** which let's us re-use the variable name
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(& secret_num) {
            Ordering::Less => println!("Low!"),
            Ordering::Greater => println!("High!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed it correctly.");
                break;
            },
        }
    }
}
