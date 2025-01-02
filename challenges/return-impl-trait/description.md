Rust's `impl Trait` syntax allows for concise and flexible code, especially when returning iterators or other complex types. By using `impl Trait`, you can abstract away concrete types while ensuring optimal performance through Rust's zero-cost abstractions.

In this challenge, you will implement a function that filters strings from a slice, returning only those that start with a given keyword. The function will return an iterator over the filtered results. This approach demonstrates how to combine Rust's iterator combinators with the `impl Trait` syntax.

## Your Task

Write a function named `filter_starts_with` that:

1. Accepts two arguments:
   - A slice of `String`
   - A `&str` keyword
2. Returns an iterator that filters and yields only the strings starting with the given keyword.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use `.iter()` to iterate over references to the strings in the slice.
- The `filter` method takes a closure to apply a filtering condition.
- Use the `starts_with` method to check if a string starts with a keyword.
- Use `move` in the closure to capture the keyword.

</details>
