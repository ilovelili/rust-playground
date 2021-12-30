use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process;

fn main() {
  println!("Guess the number!");
  println!("Please input your guess");
  let secret_number = rand::thread_rng().gen_range(1..10);
  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    println!("You guessed {}", guess);
    let guess: u32 = match guess.trim().parse() {
      Ok(number) => number,
      Err(_) => {
        println!("{} is invalid", guess);
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win");
        process::exit(0);
      }
    }
  }
}
