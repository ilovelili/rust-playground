# Structs

## Define and init a struct

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User {
  active: true,
  username: String::from("tester1"),
  email: String::from("tester1@test.com"),
  sign_in_count: 1,
};

let user2 = User {
  active: true,
  username: String::from("tester2"),
  email: String::from("tester2@test.com"),
  sign_in_count: 2,
};

user2.sign_in_count += 1;

// Using struct update syntax .., we can achieve the same effect with less code,
let user3 = User {
  username: String::from("tester3"),
  email: String::from("tester3@test.com"),
  ..user1,
}; // we can no longer use user1 since it got moved to user3
```

## Tuple structs without struct field names

```rust
struct Color(i32, i32, i32);

let red = Color(255, 0, 0);
```

3 ways to design a get_area function

```rust
// primitive
fn get_area1(width: u32, height: u32) -> u32 {
  return width * height;
}

// use tuple
fn get_area2(dimensions: (u32, u32)) -> u32 {
  return dimensions.0 * dimensions.1; // tuple index
}

// use struct
struct Rectangle {
  width: u32,
  height: u32
}

// use pointer param to borrow the struct rather than take ownership of it
fn get_area(rect: &Reatangle) -> u32 {
  return rect.width * rect.height;
}
```

## Methods

`Methods` are different from functions in that they’re defined within the context of a struct and their first param is always `self`

```rust
#[derive(Debug)] // for println! output
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    return self.width * self.height;
  }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## Automatic referencing and dereferencing

Rust has a feature called automatic referencing and dereferencing. Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or \* so object matches the signature of the method. In other words, the following are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

## Associated Functions

All functions defined within an impl block are called `associated functions` because they’re associated with the type named after the impl. We can define associated functions that don’t have `self` as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with

```rust
impl Rectangle {
    // create a square with fixed size
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name

```rust
let square = Rectangle::square(100);
```
