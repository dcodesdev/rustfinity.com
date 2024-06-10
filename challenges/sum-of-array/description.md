Arrays are a fundamental data structure in Rust that allow you to store a **fixed-size** collection of elements of the same type. A common operation is to calculate the sum of all elements in an array.

In this challenge, you will implement a function to calculate the **sum of elements in an array of integers `i32`**.

## Your task

You need to implement the function `sum_array(arr: &[i32]) -> i32` that takes a slice of integers and returns the sum of all elements.

## Requirements

- The `sum_array` function should return the sum of all elements in the array.

## Example

```rust
let arr = [1, 2, 3, 4, 5];

let sum = sum_array(&arr);
assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5 = 15
```

## Hints

- Use the `.iter()` method to iterate over the elements of the array.
- Use the `.sum()` method to calculate the sum of the elements.
