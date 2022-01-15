# Test

### Check test with the `assert!` Marco

```rust
#[derive(Debug)]
struct Rectangle {
  width: f64,
  height: f64,
}

impl Rectangle {
  fn new(width: f64, height: f64) -> Rectangle {
    Rectangle { width, height }
  }

  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

#[cfg(test)]
mod tests {
  use crate::Rectangle; // have to import the struct
  // or: use super::*;
  #[test]
  fn can_hold() {
    let larger = Rectangle::new(100.0, 30.0);
    let smaller = Rectangle::new(30.0, 10.0); // use :: for functions
    let can_hold = larger.can_hold(&smaller); // use . for methods
    assert!(can_hold);
  }
}
```

### Customized Error Message

```rust
 #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", // customize error message
            result
        );
    }
```

### #[should_panic] Attribute

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // this test should panic
    // #[should_panic(expected = "Guess value must be less than or equal to 100")] // or even panic with error message
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### Use Result<T, E> in test

We can also write tests that use Result<T, E>. We can’t use the #[should_panic] annotation on tests that use Result<T, E>. Instead, we should return an Err value directly when the test should fail.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Running Tests in Parallel or Consecutively

When you run multiple tests, by default they run in parallel using threads. If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary

```bash
cargo test -- --test-threads=1
```

## Ignore Testcase

Use #[ignore] attribute

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
