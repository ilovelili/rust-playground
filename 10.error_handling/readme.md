# Error Handling

Rust groups errors into two major categories: `recoverable` and `unrecoverable` errors. Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

## Unwinding the Stack or Aborting in Response to a Panic

By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate [profile] sections in your `Cargo.toml` file. For example, if you want to abort on panic in release mode, add this:

```toml
[profile.release]
panic = 'abort'
```

## Recoverable Errors with Result

the `Result` enum is defined as having two variants, Ok and Err

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Shortcuts for Panic on Error: `unwrap` and `expect`

If the Result value is the `Ok` variant, `unwrap` will return the value inside the Ok. If the Result is the `Err` variant, unwrap will call the `panic!` macro

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

`expect`, which is similar to `unwrap`, lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("hello.txt not found");
}
```

## Propagating Errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
  let f = File.open(file_path); // returns Result
  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e)
  }
}
```

### A Shortcut for Propagating Errors: the `?` Operator

The `?` placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
  let f = File::open(file_path)?; // if Ok, f will be the Ok value, if Err, function returns error

  let mut s = String::new();
  f.read_to_string(&s)?; // if Err, function returns, otherwise go to next line
  Ok(s);
}
```

Four versions of read_username_to_file function

```rust
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
```
