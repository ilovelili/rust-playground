## Closures

### `Fn` Traits

The `Fn` traits are provided by the standard library. All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.

Struct that holds a closure and an optional result value

```rust
struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculating: T,
  value: Option<u32>,
}

// then impl it
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
 ...
}
```

### Capturing the Environment with Closures

```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
```

Even though x is not one of the parameters of equal_to_x, the equal_to_x closure is allowed to use the x variable that’s defined in the same scope that equal_to_x is defined in.
We can’t do the same with functions

```rust
fn main() {
    let x = 4;
    fn equal_to_x(z: i32) -> bool {
        z == x
    }
    let y = 4;
    assert!(equal_to_x(y));
}
```

Compile result:

```
$ cargo run
   Compiling equal-to-x v0.1.0 (file:///projects/equal-to-x)
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:5:14
  |
5 |         z == x
  |              ^
  |
  = help: use the `|| { ... }` closure form instead
```

When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead while functions are never allowed to capture their environment

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three Fn traits as follows:

- `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
- `FnMut` can change the environment because it mutably borrows values.
- `Fn` borrows values from the environment immutably.

If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.

```rust
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x);
}
```

## Iterators

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
This code by itself doesn’t do anything useful until iterator is called

```rust
let v1 = vec![1,2,3];
let v1_iter = v1.iter();
```

### The Iterator Trait and the next Method

All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}
```

The `iter` method produces an iterator over **immutable** references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call `into_iter` instead of iter. Similarly, if we want to iterate over **mutable** references, we can call `iter_mut` instead of iter.

### Methods that Consume the Iterator

Methods that call `next` are called **consuming adaptors**, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.

```rust
#[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total1: i32 = v1_iter.sum();
        assert_eq!(total1, 6);

        // error here: use of moved valye a_iter
        let total2: u32 = v1_iter.sum();
    }
```

We aren’t allowed to use `v1_iter` after the call to `sum` because sum **takes ownership** of the iterator we call it on.

### Methods that Produce Other Iterators

Other methods defined on the Iterator trait, known as **iterator adaptors**, allow you to change iterators into different kinds of iterators. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1); // warning: must use this map iterator
```

We can use the `collect` method which consumes the iterator and collects the resulting values into a collection data type.

```rust
let v1: Vec<32> = vec![1,2,3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // Vec<_> asks Rust compiler to infer what type goes into the Vec
assert_eq!(v2, vec![2,3,4])
```

### Zip

```rust
fn main() {
    let itr1 = [100, 200, 300];
    let itr2 = [10, 20, 30];
    let iter: Vec<_> = itr1.iter().zip(itr2.iter()).collect();
    println!("{:?}", iter);
}
// [(100, 10), (200, 20), (300, 30)]
```

### Comparing Performance: Loops vs. Iterators

The iterator version is slightly faster than loop

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```
