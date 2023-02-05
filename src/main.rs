use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("------- Guess the number -------");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Type a number between 1 and 10");

        let mut number: String = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error reading the line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("The number was {secret_number}, you win!");
                break;
            }
        }
    }
}
