In Rust, variables are immutable by default. This means that once a value is bound to a variable, it cannot be changed. This is a key feature of Rust that helps ensure safety and prevent bugs by avoiding unexpected changes to values.

In this challenge, you will declare and use immutable variables in Rust. You will be given a function where you need to declare variables and use them to perform specific operations. The goal is to get comfortable with the concept of immutability and understand how to use immutable variables effectively.

## Your task

Your task is to complete the given function by declaring immutable variables and using them to perform specific operations. You need to calculate the area of a rectangle and return the result.

## Requirements

- Declare two immutable variables, `width` and `height`, and assign them values.
- Calculate the area of the rectangle using the formula `area = width * height`.
- Return the calculated area.

## Example

```rust
fn calculate_area() -> u32 {
    // Declare width and height
    let width = 10;
    let height = 5;

    // Calculate area
    let area = width * height;

    // Return area
    area
}

assert_eq!(calculate_area(), 50);
```

## Hints

- Use the `let` keyword to declare variables.
- Remember that variables declared with `let` are immutable by default.
- Use multiplication `*` to calculate the area.
