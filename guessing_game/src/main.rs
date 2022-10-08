use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let generate_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your input : ");

        let mut guess = String::new();
        let msg = "Failed to read line";
        io::stdin().read_line(&mut guess).expect(msg);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&generate_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it correct");
                break;
            }
        }
    }
}
