# Common Concepts

## Date Types - Scalar Type & Compound Type

### Scalar Type (Value type)

- Int
- Float
  - f64 (default)
  - f32
- Boolean
- Character

Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII

```rust
let a = 'z';
let a = 'ℤ';
let a = '😻';
```

### Compound Type

Compound types can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

#### Tuple

Tuples have a fixed length: once declared, they cannot grow or shrink in size

```rust
let tup: (i64, f64, bool) = (1, 2.3, true);
let (_, y, _) = tup;
println!("y is {}", y);
```

#### Array

Unlike Tuple type, each element in array must has the same type

```rust
let a = [1,2,3];
let a:[i32;5] = [1,2,3,4,5];
```

Array with same value

```rust
let a = [3;5] // same with let a = [3,3,3,3,3];
```

## Functions

Rust code uses `snake case` as the conventional style for function and variable names

```rust
fn my_function() -> u32 {
  1;
}
```

### Statements and expressions

Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

```rust
let a = 1; // statement

{
    let x = 3;
    x + 1
} // this block is an expression

```

## Control Flows

### If

- if / else
- if / else if

### Loop

defines an infinite loop. Use `break` to exit the loop

```rust
let mut i = 1;
loop {
  i = i + 1;
  println!("again!");

  if (i > 100) {
    break;
  }
}
```

Return values from loop

```rust
let mut counter = 0;
let a = loop {
  counter += 1;
  if counter == 10 {
      break counter * 2;
  }
}
```

### While

Conditional loops with while

```rust
let mut i = 100;
while (i < 100) {
  println!("again");
  i += 1;
}
```

### Loop thru an array using `for ... in ...`

```rust
let a: [i64; 3] = [1, 2, 3];
for x in a {
  println!("value is {}", x);
}
```

### Range type

`Range` is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number. Use `..` to define a range. `rev` can be used to reverse a range

```rust
for number in (1..5).rev() {
  println!("value is {}", number);
}
```
