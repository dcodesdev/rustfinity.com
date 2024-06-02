In this challenge, you will practice using the `for` loop in Rust to iterate over a range of numbers. The goal is to sum all the even numbers within a given range.

The `for` loop in Rust is a powerful construct that allows you to iterate over collections, ranges, and iterators. It is commonly used for tasks such as summing numbers, collecting items, and more. By utilizing the `for` loop, you can write concise and readable code to perform repetitive tasks.

## Your task

Your task is to implement a function `sum_of_evens` that takes two arguments, `start` and `end`, both of which are `i32`. The function should return the sum of all even numbers within the range `[start, end]` inclusive. If `start` is greater than `end`, the function should return `0`.

## Requirements

- Use a `for` loop to iterate over the range from `start` to `end`.
- Sum only the even numbers within the specified range.
- If `start` is greater than `end`, return `0`.

## Example

```rust
let result = sum_of_evens(1, 10);
assert_eq!(result, 30); // 2 + 4 + 6 + 8 + 10 = 30

let result = sum_of_evens(5, 5);
assert_eq!(result, 0); // 5 is not even

let result = sum_of_evens(10, 1);
assert_eq!(result, 0); // start is greater than end
```
