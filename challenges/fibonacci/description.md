The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding ones, usually starting with 0 and 1. The sequence starts with 0 and 1, and then each number is the sum of the two preceding ones.

You are given a function `fibonacci(n: u32) -> u32` that takes a number `n` and returns the `n`-th number in the Fibonacci sequence.

Your job is to implement the function meeting the following requirements:

- If given `n` is 0, the function should return 0.
- If given `n` is 1, the function should return 1.
- If given `n` is greater than 1, the function should return the sum of the two preceding numbers.

## Example

Here are some examples of how the behavior of the function should look like:

```rust
assert_eq!(fibonacci(0), 0); // Returns 0
assert_eq!(fibonacci(1), 1); // Returns 1
assert_eq!(fibonacci(2), 1); // Returns 1 + 0 = 1
assert_eq!(fibonacci(3), 2); // Returns 1 + 1 = 2
assert_eq!(fibonacci(4), 3); // Returns 2 + 1 = 3
assert_eq!(fibonacci(5), 5); // Returns 3 + 2 = 5
assert_eq!(fibonacci(6), 8); // Returns 5 + 3 = 8
assert_eq!(fibonacci(7), 13); // Returns 8 + 5 = 13
```
