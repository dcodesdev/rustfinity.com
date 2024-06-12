Functions are a fundamental building block in Rust, as in any programming language. They allow you to encapsulate logic and reuse code, making your programs more modular and easier to understand. In this challenge, you will define and implement a series of simple functions that perform basic operations.

## Your task

Your task is to define three functions:

1. `add(a: i32, b: i32) -> i32` - This function should return the sum of `a` and `b`.
2. `subtract(a: i32, b: i32) -> i32` - This function should return the difference between `a` and `b`.
3. `multiply(a: i32, b: i32) -> i32` - This function should return the product of `a` and `b`.

You need to complete these functions so that they correctly perform the specified operations.

## Example

```rust
let result = add(2, 3);
assert_eq!(result, 5);

let result = subtract(5, 3);
assert_eq!(result, 2);

let result = multiply(2, 3);
assert_eq!(result, 6);
```

## Hints

- You can define a function in Rust using the `fn` keyword followed by the function name, parameters in parentheses, and the return type.
- Use the `return` keyword or **just the last expression in the function body** to return a value.
- Make sure you use the `pub` keyword to make your functions public so they can be accessed from other modules.
- Make sure your function signatures match the required signatures exactly.
