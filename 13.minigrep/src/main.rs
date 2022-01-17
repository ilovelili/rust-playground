use minigrep::Config;
use std::{env, process};

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    // eprintln outputs to std error instead of std output
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  // We use if let to check whether run returns an Err value
  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
