Slices are one of Rust's most versatile and powerful tools for working with collections of data. A slice is a view into a contiguous sequence of elements in a collection, allowing you to operate on a subset of the data without needing to copy it.

In this challenge, you will work with slices to find the largest number in a given range of a slice. Slices allow you to efficiently access and process subsets of arrays and vectors. Your task is to write a function that handles edge cases like empty slices and returns the largest number where possible.

## Your task

Implement the function `find_largest_in_slice(slice: &[i32]) -> Option<i32>` that takes an immutable slice of integers and returns the largest integer in the slice. If the slice is empty, return `None`.

Slices provide various methods that can help you work efficiently, such as iterators and bounds checking.

### Requirements

- The function should return `Some(i32)` with the largest element in the slice if the slice is not empty.
- If the slice is empty, the function should return `None`.
- The function can not modify the input slice.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use the `.iter()` method on slices to create an iterator over the elements.
- Consider using the `.max()` method provided by iterators to find the largest value.
- If using `.max()`, remember to handle the `Option` it returns, as an empty iterator will yield `None`.

</details>
