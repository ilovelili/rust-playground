use std::collections::HashMap;

fn main() {
  let mut vec = vec![1, 2, 3];
  vec.push(4);
  vec.push(5);

  let first = vec.get(0);

  // This error is due to the way vectors work:
  // adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
  // if there isn’t enough room to put all the elements next to each other where the vector currently is
  // vec.push(6);

  println!("The first element is: {:?}", first);

  let mut v = vec![1, 2, 3];
  for i in &mut v {
    *i += 10; // *i instead of i
  }
  // {:?} formats the value supports anything that implements Debug
  println!("v is {:?}", v);

  let a = "hello world";
  let b = a.to_string();
  println!("{}", b);

  println!("{}", String::from("السلام عليكم"));
  println!("{}", String::from("Dobrý den"));
  println!("{}", String::from("Hello"));
  println!("{}", String::from("שָׁלוֹם"));
  println!("{}", String::from("नमस्ते"));
  println!("{}", String::from("こんにちは"));
  println!("{}", String::from("안녕하세요"));
  println!("{}", String::from("你好"));
  println!("{}", String::from("Olá"));
  println!("{}", String::from("Здравствуйте"));
  println!("{}", String::from("Hola"));

  let mut a = String::from("hello");
  let b = a.push_str("world");
  let c = String::from("abc");
  let d = a + &c;

  println!("{:?}, {:?}, {:?}", b, c, d);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);

  let s1 = String::from("Здравствуйте");
  let s2 = &s1[0..4];
  println!("string slice is {}", s2);

  let s1 = String::from("Здравствуйте");
  for c in s1.chars() {
    println!("char is {}", c);
  }

  for b in s1.bytes() {
    println!("byte is {}", b);
  }

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let s1 = String::from("Blue");
  let s2 = String::from("Yellow");
  scores.insert(s1, 10);
  scores.insert(s2, 50);

  println!("scores: {:?}", scores);
  // println!("{}", s1); // return error since s1 has moved to hashmap

  // let key = String::from("Blue");
  // let v = scores.get(&key);
  // println!("{:?}", v);

  // for (k, v) in scores {
  //   println!("key is {}, value is {}", k, v);
  // }

  for color in ["Blue", "Green", "Yellow"] {
    let result = scores.entry(String::from(color)).or_insert(100);
    println!("color {}, result {}", color, result)
  }

  for (k, v) in scores {
    println!("key is {}, value is {}", k, v);
  }

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for text in text.split_whitespace() {
    let count = map.entry(text).or_insert(0);
    *count += 1
  }

  println!("{:?}", map);
}
