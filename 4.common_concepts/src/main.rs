fn main() {
  let mut x = 3;
  println!("x is {}", x);
  x = 5;
  println!("x is {}", x);

  let tup: (i64, f64, bool) = (1, 2.3, true);
  let (_, y, _) = tup;
  println!("y is {}", y);

  let mut buffer = itoa::Buffer::new();
  println!("one is {}", buffer.format(one()));

  let loop_value = value_from_loop();
  println!("value from loop: {}", buffer.format(loop_value));

  let a: [i64; 3] = [1, 2, 3];
  for x in a {
    println!("value is {}", x);
  }

  for number in (1..5).rev() {
    println!("value is {}", number);
  }
}

fn one() -> u64 {
  return 1;
}

fn value_from_loop() -> u64 {
  let mut counter = 0;
  return loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
}
