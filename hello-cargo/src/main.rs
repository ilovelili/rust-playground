fn main() {
    let s = String::from("Hello, world!");
    print(&s);
    print(&s);

    let mut k = String::from("Hello");
    mutable_print(&mut k);
    mutable_print(&mut k);

    print_take_ownership(s);
    // print_take_ownership(s); // can't call s here since s has been moved (it's not a copy)
}

fn print(s: &String) {
    println!("{}", s);
}

fn mutable_print(s: &mut String) {
    s.push_str(", ade");
    println!("{}", s);
}

fn print_take_ownership(s: String) {
    println!("{}", s);
}
