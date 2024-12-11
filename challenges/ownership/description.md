Rust's ownership model is one of its most unique and powerful features, ensuring **memory safety** without needing a garbage collector. Ownership in Rust is governed by a set of rules that the compiler checks at compile time. Understanding these rules is crucial for writing efficient and safe Rust code.

## Ownership Basics

In Rust, each value has a variable that's called its **owner**. There can only be one owner at a time, and when the owner goes out of scope, the value is dropped. Here are the basic rules of ownership:

1. Each value in Rust has a variable that's called its **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value will be **dropped** (no longer valid).

### Example

```rust
{
    // s is the owner of the String
    let s = String::from("hello");
} // s goes out of scope and "hello" is dropped
```

## Borrowing

Rust allows you to create **references** to a value, which lets you access it without taking ownership. This is called **borrowing**. Borrowing can be **immutable** or **mutable**.

### Immutable References

You can create multiple **immutable references** to a value, but you cannot have a mutable reference while immutable references exist. This allows you to read from the value without changing it.

### Example

```rust
fn main() {
    let s1 = String::from("hello");

    // Borrow s1 as immutable
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

// s is an immutable reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Using `&` to Create References

In the example above, `&s1` creates an immutable reference to `s1`. This means that `calculate_length` borrows `s1` but does not take ownership of it. The `&` symbol is used to denote a reference in Rust. This allows the function to access the value without taking ownership, which means `s1` can still be used after the function call.

Similarly, in the function signature `fn calculate_length(s: &String) -> usize`, `&String` indicates that the parameter `s` is an immutable reference to a `String`. This allows the function to read from the `String` without modifying it or taking ownership.

## Challenge

In this challenge, you will create a function `calculate_length` that takes an immutable reference to a `String`, calculates its length, and returns the length.

The task is designed to help you understand the concepts of ownership and immutable borrowing in Rust.

## Requirements

- The `calculate_length` function should take **an immutable reference** to the input `String` and return its length.

### Example

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
assert_eq!(len, 5);
```

## Hints

- The `String` type in Rust has a method `len()` which returns its length.
