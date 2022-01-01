# Enums

## Define an enum

```rust
enum IPAddr {
  v4,
  v6,
}

// use :: to get the instance
let ipv4 = IPAddr::v4;
```

We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data directly into each enum variant.

```rust
enum IPAddr {
  v4(String),
  v6(String),
}

let ipv4 = IPAddr::v4(String::from("127.0.0.1"));
```

Enum with a wide variety of types

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

We can also define methods for enums

```rust
impl Message {
  fn call(&self) {
    // do something
  }
}

let msg = Message::Write(String::from("hi"));
msg.call();
```

## The Option Enum and Its Advantages Over Null Values

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`

```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
let some_number = Some(5); // The type of some_number is Option<i32>.
let some_string = Some("a string"); // The type of some_string is Option<&str>
let absent_number: Option<i32> = None;
```

When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having Option<T> any better than having null?

In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. In other words, you have to convert an Option<T> to a T before you can perform T operations with it.

For example, this code won’t compile because it’s trying to add an i8 to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // error
```

## The `match` control flow

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

### `Match` with `Option<T>`

```rust
fn plus_one(x: Option<u32>) -> Option<u32> {
  match x {
    Some(i) => Some(i + 1),
    None => None,
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Catch-all Patterns and the \_ Placeholder

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // If we don’t need to use the value in that case, so we can change our code to use _ instead of the variable named other
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

### Or we ignore the other options

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // do nothing just to cover matches (matches are exhaustive)
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

## Concise Control Flow with `if let`

The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("max is {}", max);
}

// same as match
match config_max {
  Some(max) => println!("max is {}", max),
  _ => (),
}
```
