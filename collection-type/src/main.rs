use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third = v.get(2); // Option<&i32> type
    println!("{:?}", third.unwrap());

    for i in v {
        println!("{}", i);
    }

    // append a string
    let mut s = String::from("foo");
    s.push_str(",bar");
    s += "xx";
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here so it can no longer be accessed.
    println!("{}", s3);

    // string index failure
    let hello = "Здравствуйте";
    let s = &hello[0..2];
    println!("{}", s);
    // this will fail since Unicode scalar value takes 2 bytes of storage. So byte index 1 is not a char boundary
    // let s = &hello[0..1];

    // interating over string
    for c in "नमस्ते ".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते ".bytes() {
        println!("{}", b);
    }

    // hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
