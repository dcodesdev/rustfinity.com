In Rust, the unit type `()` is a type that has exactly one value, also written as `()`. It is used to indicate the absence of a meaningful value and is often seen in functions that do not return a value.

In this challenge, you will implement a function that prints a message and returns the unit type.

## Your task

You need to implement the function `print_message() -> ()` that prints `"Hello, Rust!"` to the console and returns the unit type.

## Requirements

- The `print_message` function should print `"Hello, Rust!"` to the console.
- The function should return the unit type `()`.

## Example

```rust
let result = print_message();
assert_eq!(result, ());
```
