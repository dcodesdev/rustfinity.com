Rust's ownership model is one of its most unique and powerful features, ensuring **memory safety** without needing a garbage collector. Ownership in Rust is governed by a set of rules that the compiler checks at compile time. Understanding these rules is crucial for writing efficient and safe Rust code.

## Ownership Basics

In Rust, each value has a variable that's called its **owner**. There can only be one owner at a time, and when the owner goes out of scope, the value is dropped. Here are the basic rules of ownership:

1. Each value in Rust has a variable that's called its **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value will be **dropped** (no longer valid).

### Example

```rust
{
    let s = String::from("hello"); // s is the owner of the String
} // s goes out of scope and "hello" is dropped
```

## Borrowing

Rust allows you to create **references** to a value, which lets you access it without taking ownership. This is called **borrowing**. Borrowing can be **immutable** or **mutable**.

### Immutable References

You can create multiple **immutable references** to a value, but you cannot have a mutable reference while immutable references exist. This allows you to read from the value without changing it.

## Example

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // borrow s1 as immutable
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is an immutable reference to a String
    s.len()
} // s goes out of scope here, but since it does not have ownership of the String, nothing happens
```

## Challenge

In this challenge, you will create a function `calculate_length` that takes an immutable reference to a `String`, calculates its length, and returns the length.

The task is designed to help you understand the concepts of ownership and immutable borrowing in Rust.

## Requirements

- The `calculate_length` function should take **an immutable reference** to the input `String` and return its length.

## Example

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
assert_eq!(len, 5);
```

## Hints

- The `String` type in Rust has a method `len()` which returns its length.
