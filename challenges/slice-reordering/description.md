# Slice Reordering Puzzle

Now that you have an overview of slices, let's dive into a more challenging problem that will test your understanding of mutable slices and efficient in-place operations.

Slices in Rust allow you to work on portions of data safely and efficiently. When working with mutable slices (`&mut [T]`), you can modify the underlying data while respecting Rust's strict borrowing rules. In this challenge, your task is to reorder a slice based on specific rules and pad the remaining space with a default value.

Your goal is to design a function that:

1. Reorders a slice based on a custom condition, separating elements into two groups while maintaining relative order within each group.
2. Ensures all remaining elements after the reordered groups are replaced with a specified padding value.
3. Performs all operations in-place and efficiently.

## Your Task

Write a function `reorder_and_pad` with the following signature:

```rust
pub fn reorder_and_pad(slice: &mut [i32], is_group_one: fn(i32) -> bool, pad_value: i32) -> usize;
```

- **`slice`**: A mutable slice of integers.
- **`is_group_one`**: A function pointer that takes an integer and returns `true` if it belongs to Group 1, otherwise `false`.
- **`pad_value`**: The value used to pad the remaining elements.

The function must:

1. Reorder the slice so that elements satisfying `is_group_one` appear first, followed by all other elements.
2. Maintain the relative order of elements in each group.
3. Replace all remaining elements in the slice (after the reordered groups) with `pad_value`.
4. Return the count of elements in Group 1.

### Constraints:

- `slice.len() > 0` (non-empty slices).
- The operation must be performed **in-place** without extra allocations.
- No unsafe code or external libraries are allowed.

## Example

```rust
fn is_negative(n: i32) -> bool {
    n < 0
}

let mut data = [3, -2, 4, -5, 1];
let count = reorder_and_pad(&mut data, is_negative, 0);

assert_eq!(count, 2);
assert_eq!(data, [-2, -5, 3, 4, 0]);
```

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

let mut data = [1, 2, 3, 4, 5];
let count = reorder_and_pad(&mut data, is_even, -1);

assert_eq!(count, 2);
assert_eq!(data, [2, 4, 1, 3, -1]);
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

1. **Reordering elements**: You can use a loop to iterate through the slice and track indices for each group.
2. **Maintaining relative order**: A two-pass solution with additional temporary indices can help maintain order.
3. **Padding values**: Use slice slicing techniques to fill the unused portion of the slice with the `pad_value`.
4. **Counting group members**: Track the number of elements satisfying the condition as you process the slice.

</details>
