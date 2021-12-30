# Guessing Game

## mut

In Rust, variables are immutable by default. So we need to add `mut` keyword for mutable variables

```rust
let apple = 5;

let mut banana = 1;
banana = 3;
```

## expect

The right way to suppress the warning is to actually write error handling, but because we just want to crash this program when a problem occurs, you can use `expect`

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

## println! placeholder

```rust
println!("You guessed: {}", guess);
```

## Cargo edit

[Cargo edit Github](https://github.com/killercup/cargo-edit)

```bash
sudo apt install libssl-dev pkg-config
cargo install cargo-edit
```

Then we can use `cargo-add`, `cargo-rm`, `cargo-set-version`, `cargo-upgrade` ...

## match

Match syntax

```rust
use std::cmp::Ordering;

match a.cmp(&b) {
  Ordering::Less => println!("too small"),
  Ordering::Greater => println!("too big"),
  Ordering::Equal => println!("equal")
}

```
