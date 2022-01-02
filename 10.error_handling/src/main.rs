use std::fs::{self, File};
use std::io::Read;
use std::io::{self, ErrorKind};

fn main() {
  // unwrap will call panic! macro if file not found
  // File::open("hello.txt").unwrap();

  File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      // unwrap_or_else closure
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });

  println!(
    "{}",
    read_username_from_file_v1("hello.txt").expect("file not found")
  );
}

fn read_username_from_file_v1(file_path: &str) -> Result<String, io::Error> {
  let f = File::open(file_path); // returns Result
  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();
  return match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  };
}

fn read_username_from_file_v2(file_path: &str) -> Result<String, io::Error> {
  let mut f = File::open(file_path)?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  return Ok(s);
}

fn read_username_from_file_v3(file_path: &str) -> Result<String, io::Error> {
  let mut s = String::new();
  File::open(file_path)?.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file_v4(file_path: &str) -> Result<String, io::Error> {
  fs::read_to_string(file_path)
}
