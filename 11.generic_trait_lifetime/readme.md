# Generics, Traits and LifeTime

## Trait

`Trait` is interface

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

### Implementing a Trait on a Type

syntax: **impl {{trait}} for {{struct}}**

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}: {}", self.author, self.content)
  }
}

let article = NewsArticle {
  headline: String::from("today's article"),
  author: String::from("min"),
  content: String::from("today's stock price"),
};

println!("{}", article.summarize());
```

### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

impl Summary for NewsArticle {}
println!("{}", article.summarize());
```

### Traits as Parameters

We can define a notify function that calls the summarize method on its item parameter, which is of some type that implements the Summary trait.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:

```rust
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
```

### Specifying Multiple Trait Bounds with the + Syntax

We can also specify more than one trait bound using the `+` syntax

```rust
pub fn notify(item: &impl(Summary + Display)) {

}

// or

pub fn notify<T: Summary + Display>(item: &T) {

}
```

### Clearer Trait Bounds with where Clauses

```rust
pub fn notify<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
```

### Returning Types that Implement Traits

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type Pair<T> in Listing 10-16 always implements the new function. But Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self(x, y)
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

## Lifetimes

The main aim of `lifetimes` is to prevent dangling references

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. Most people use the name 'a. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.

```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.

function without explicit lifetime:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
*/
```

function with explicit lifetime:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in

Lifetimes on function or method parameters are called `input lifetimes`, and lifetimes on return values are called `output lifetimes`

The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error

### Lifetime Elision Rule

- The first rule is that **each parameter that is a reference gets its own lifetime parameter**. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

- The second rule is **if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters**: fn foo<'a>(x: &'a i32) -> &'a i32

- The third rule is **if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters**.

```rust
fn first_word(s: &str) -> &str {
// first rule:
// fn first_word<'a>(s &'a str) -> &str
// then second rule:
// fn fisrt_word<'a>(s &'a str) -> &'a str
```

```rust
fn longest(x: &str, y: &str) -> &str {
// first rule:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
// second rule not applied since there are multiple params
// third rule not applied since it's not a method
// => compile error
```

### The Static Lifetime

`static` means that this reference can live for the entire duration of the program

```rust
let s: &'static str = "I have a static lifetime.";
```

## Put Generic, Trait, Lifetime together

```rust
use std::fmt::Display;

fn longest<'a, T>(a &'a str, b &'a str, ann &T) -> &'a str where T: Display {
  println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
