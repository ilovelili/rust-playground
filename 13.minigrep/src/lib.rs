use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  // we dont want to return the same lifetime as param so we add 'static in return
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    // ignore the 1st arg
    args.next();

    // query as 2nd arg
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    // filename as 3rd arg
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a filename"),
    };

    Ok(Config { query, filename })
  }
}

// dyn -> dynamic type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  /*
  The `?` placed after a Result value is defined to work in almost the same way as the
  match expressions we defined to handle the Result values.
  If the value of the Result is an Ok, the value inside the Ok will get returned from
  this expression,
  and the program will continue. If the value is an Err, the Err will be returned from
  the whole function as if we had used the return keyword so the error value gets propagated
  to the calling code.
  */
  let query = config.query;
  let contents = fs::read_to_string(config.filename)?;

  for line in search(&query, &contents) {
    println!("found: {}", line)
  }

  Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  for line in content.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
