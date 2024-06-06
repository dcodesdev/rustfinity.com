In this challenge, you will implement a function to calculate the **factorial** of a given **non-negative integer**. The factorial of a number `n` (denoted as `n!`) is **the product of all positive integers less than or equal to `n`**. For example, the factorial of 5 is `5! = 5 * 4 * 3 * 2 * 1 = 120`.

Factorials grow very quickly, so it is important to handle large numbers appropriately. Rust's standard library provides the `u128` type, which can handle very large integers. Your task is to implement a function that calculates the factorial of a given `u32` number and returns the result as a `u128`.

## Your task

Implement a function `factorial(n: u32) -> u128` that calculates the factorial of the given non-negative integer `n`. Use early returns to handle the base case when `n` is 0, since `0!` is defined as 1. For other values of `n`, use a **loop** to calculate the factorial.

## Requirements

- The function should take a single argument `n` of type `u32` and return a `u128`.
- Use early returns to handle the case when `n` is `0`.
- For other values of `n`, use a loop to calculate the factorial.

## Example

```rust
let result = factorial(5);
assert_eq!(result, 120);

let result = factorial(10);
assert_eq!(result, 3628800);

let result = factorial(0);
assert_eq!(result, 1);
```

## Hints

- Remember that Rust has a `u128` type which can hold **very large numbers**. This will be useful for calculating large factorials.
- Use the `return` statement to handle the base case of `0`!.
- A `for` loop or `while` loop can be used to multiply the numbers from `1` to `n`.
- Consider using an iterative approach rather than a recursive one to avoid potential **stack overflow** issues for large `n`.
