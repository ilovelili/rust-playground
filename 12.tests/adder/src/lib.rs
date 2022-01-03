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
    let smaller = Rectangle::new(30.0, 10.0);
    let can_hold = larger.can_hold(&smaller);
    assert!(can_hold, "can hold failed!"); // use 2nd param to customize error msg
  }

  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
}
