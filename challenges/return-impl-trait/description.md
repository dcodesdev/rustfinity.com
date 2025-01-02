Rust's `impl Trait` syntax enables you to abstract over return types, making your code more flexible while preserving type safety. This challenge focuses on using `impl Trait` to return an iterator over a filtered slice.

In this task, you will implement a function that filters even numbers from a slice and returns an iterator over the filtered results. This approach is efficient and leverages Rust's iterator combinators.

## Your Task

Write a function named `filter_even_numbers` that:

1. Takes a slice of integers (`&[i32]`) as input.
2. Returns an iterator (`impl Iterator<Item = &i32>`) that filters and yields only the even numbers.

### Requirements

1. The function must accept a slice of integers (`&[i32]`).
2. It must return an `impl Iterator<Item = &i32>`.
3. Use the `filter` method to include only even numbers.

### Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `.into_iter()` method to create an iterator over the slice.
- Use the `filter` method to filter out odd numbers.
- `%` can determine whether a number is even.

</details>
