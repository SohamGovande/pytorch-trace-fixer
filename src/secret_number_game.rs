use std::cmp::Ordering;
use std::io;
use rand::Rng;
use crate::secret_number_game;

pub(crate) fn secret_number_game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("guess the number {}", secret_number);

    loop {
        println!("input: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("u guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("good");
                break
            },
            Ordering::Greater => println!("too big"),
        }
    }
}