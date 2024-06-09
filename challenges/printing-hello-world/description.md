It's a convention to start learning a new programming language by writing a program that prints `"Hello, World!"` to the console.

In Rust, we can do this using the `println!` macro. The `!` indicates that this is a **macro** rather than a **function**.

## What is a macro?

**A macro is a way to write code that writes other code.** **Macros** are a powerful feature of Rust that allow you to write code that writes code. This is useful for **metaprogramming**, which is the process of writing code that writes code.

## println!

The `println!` macro is used to print text to the console. It is similar to the `println` function in other programming languages, but it is a **macro** in Rust.

Here's how to print `Hello` to the console using the `println!` macro:

```rust
fn main() {
    println!("Hello");
}
```

## Your task

Write a program that prints `"Hello, World!"` to the console using the `println!` macro.

## Expected Output

```rust
Hello, World!
```
