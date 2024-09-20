Welcome to the first challenge of the Rustfinity DSA track!

First challenge is going to be a simple one.

You will reverse the order of elements in an array of integers.

## Your task

Implement the function `reverse_array(arr: &mut [i32])`. The function will reverse the array in place, without creating a new array.

## Requirements

- Reverse the array in place.
- Handle arrays of any size, including empty arrays.

## Example

```rust
let mut arr = [1, 2, 3, 4, 5];
reverse_array(&mut arr);
assert_eq!(arr, [5, 4, 3, 2, 1]);

let mut arr: [i32; 0] = [];
reverse_array(&mut arr);
assert_eq!(arr, []);
```
