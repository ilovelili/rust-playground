use std::iter::Iterator;
use std::thread;
use std::time::Duration;

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
    println!("calculating ...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_closure.value(intensity));
    println!("Next, do {} situps!", expensive_closure.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculating: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculating: T) -> Cacher<T> {
    Cacher {
      calculating,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculating)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
  }

  #[test]
  fn test_iterator_sum() {
    let a = vec![1, 2, 3];
    let a_iter = a.iter();
    let sum1: u32 = a_iter.sum();
    println!("sum1 is {}", sum1);
    // // use of moved valye a_iter
    // let sum2: u32 = a_iter.sum();
    // println!("sum2 is {}", sum2);
  }

  #[test]
  fn test_collect() {
    let v1 = vec![1, 2, 3];
    // Vec<_> asks Rust compiler to infer what type goes into the Vec
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4])
  }
}
