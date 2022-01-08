use rand::{self, Rng};
use std::{cmp::Ordering, io::stdin};

pub fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", random_number);

    loop {
        let mut guessed_number = String::new();
        stdin()
            .read_line(&mut guessed_number)
            .expect("error reading stdin");
        let guessed_number: i32 = match guessed_number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("{} is not a number", &guessed_number.trim());
                continue;
            }
        };

        match guessed_number.cmp(&random_number) {
            Ordering::Equal => {
                println!("Equal you won");
                break;
            }
            Ordering::Greater => println!("number is greater"),
            Ordering::Less => println!("number is lower"),
        }
    }
}
