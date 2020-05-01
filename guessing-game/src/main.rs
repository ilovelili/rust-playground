use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Guess the number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read a line");
        let guess_number = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        }; // trim ...

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
