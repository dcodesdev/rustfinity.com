In this challenge, you will implement a simple countdown timer using a **while loop**. The goal is to create a function that takes an integer `n` and returns a vector containing the countdown from `n` to 0.

## Your task

Write a function `countdown(n: u32) -> Vec<u32>` that takes a non-negative integer `n` and returns a vector with the numbers from `n` to 0.

## Requirements

- Use a `while` loop to implement the countdown.
- Ensure the function handles the case where `n` is 0 correctly.
- The function should return a vector with each number in the countdown sequence.

## Example

```rust
let result = countdown(3);
assert_eq!(result, vec![3, 2, 1, 0]);

let result = countdown(0);
assert_eq!(result, vec![0]);
```
