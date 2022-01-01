fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let s = &String::from("hello world");
    let f = first_world(s, ' ');
    println!("first world {}", f);

    // s.clear(); // error! since string slice f is still valid and immutable
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_world(s: &String, t: char) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == t as u8 {
            return &s[..i];
        }
    }
    return &s[..];
}
