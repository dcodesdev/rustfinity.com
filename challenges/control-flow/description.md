Control flow is a fundamental concept in programming that allows you to dictate how your code executes based on certain conditions. In Rust, one of the ways to control the flow of your program is by using `if-else` statements.

In this challenge, you will implement a function that checks whether a number is **positive**, **negative**, or **zero**. Depending on the value, your function should return a corresponding string message.

## Your task

Your task is to complete the function `check_number_sign` that takes an **integer `i32`** as input and returns a `String` indicating whether the number is `"positive"`, `"negative"`, or `"zero"`.

## Requirements

- If the number is greater than zero, return `"positive"`.
- If the number is less than zero, return `"negative"`.
- If the number is equal to zero, return `"zero"`.

## Example

```rust
let result = check_number_sign(10);
assert_eq!(result, "positive");

let result = check_number_sign(-5);
assert_eq!(result, "negative");

let result = check_number_sign(0);
assert_eq!(result, "zero");
```

## Hints

- You can use the `if`, `else if`, and `else` keywords to implement the control flow.
- Remember to return the result as a `String`.
