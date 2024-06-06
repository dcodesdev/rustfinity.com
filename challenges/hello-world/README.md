Welcome to Rustfinity, we're glad you've decided to improve your Rust skills. In this challenge, you will be asked to write a simple function to return the string slice `"Hello, World!"`.

String slices are references to a sequence of bytes stored in memory, primarily used to store text data. In Rust, string slices are written as `&str`. Adding `'static to &str` indicates that the string slice is stored in the program's memory for its entire program's lifetime.

In this challenge, you will write a function that returns a string slice of type `&'static str` with the value `"Hello, World!"`.

## Returning values from functions in Rust

In Rust, there are two ways to return values from functions. The first way is to use the `return` keyword, followed by the value you want to return, and then a semicolon. The second way is to omit the semicolon and the `return` keyword; the value will then be returned implicitly.

Here's an example of a function that returns a string slice:

```rust
fn returns_text() -> &'static str {
  "Some, text!"
}
```

Or using the `return` keyword:

```rust
fn returns_text() -> &'static str {
  return "Some, text!";
}
```
