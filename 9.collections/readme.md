# Collections

Unlike the built-in array and tuple types, the data `collections` point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs

## Vector

`Vectors` allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type

```rust
let vec: Vec<u32> = Vec::new();
```

Use `vec!` macro to create a vector

```rust
let mut vec = vec![1, 2, 3];
vec.push(5);
vec.push(6);
```

Interate values of a vector

```rust
let mut v = vec![1, 2, 3];

for i in &mut v {
  *i += 50; // *i instead of i
}
```

### Using an Enum to Store Multiple Types

Vectors can only store values of the same type. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum

```rust
enum SpreadSheetCell {
  Int(i64),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadSheetCell::Int(3),
  SpreadSheetCell::Float(4.5),
  SpreadSheetCell::Text(String::from("hello")),
];
```

## String

`Strings` are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text

Rust string (inluding String and &str) are utf-8 encoded

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

### to_string

`to_string` method is available on any type that implements the `Display` trait

```rust
let a = "hello world"; // &str
let b = a.to_string(); // String
```

### concat string

```rust
// 1. use push_str
let mut a = String::from("hello");
let b = a.push_str("world"); // b is str

// 2. use + &str
let c = String::from("abc");
let d = a + &c; // + &str

// 3. use format!
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### string index

```rust
let s1 = String::from("hello");
let h = s1[0]; // error: the type `String` cannot be indexed by `{integer}`
```

Rust `String` don’t support indexing...

A `String` is a wrapper over a Vec\<u8\>

```rust
let hello = String::from("Hola"); // string len will be 4
let hello = String::from("Здравствуйте"); // string len will be 24, that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage

let answer = &hello[0]; // What should the value of answer be? Should it be З, the first letter? When encoded in UTF-8, the first byte of З is 208 and the second is 151, so answer should in fact be 208, but 208 is not a valid character on its own. To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t support string indexing

// bytes for Здравствуйте
// d0 97 d0 b4 d1 80 d0 b0 d0 b2 d1 81 d1 82 d0 b2 d1 83 d0 b9 d1 82 d0 b5
```

Indexing a string is not a good idea in Rust, but Rust does support string slicing:

```rust
let s1 = String::from("Здравствуйте");
let s2 = &s1[0..4]; // we mentioned that each of these characters was 2 bytes, which means s will be Зд
```

### Methods for Iterating Over Strings

```rust
let s1 = String::from("Здравствуйте");
for c in s1.chars() {
  println!("char is {}", c);
}

for b in s1.bytes() {
  println!("byte is {}", b);
}
```

## Hashmaps

Create a hashmap using `new` and add elements using `insert`. Like `vectors`, hash maps store their data on the heap. Hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### Only Inserting a Value If the Key Has No Value

It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it. Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return value of the entry method is an enum called `Entry` that represents a value that might or might not exist. The `or_insert` method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 50);
scores.entry(String::from("Blue")).or_insert(50);
println!("scores {:?}", scores);
```

Count word appearance

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();
for text in text.split_whitespace() {
  let count = map.entry(text).or_insert(0);
  *count += 1
}

println!("{:?}", map);
```
