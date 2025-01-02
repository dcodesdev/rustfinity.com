Rust's `impl Trait` syntax allows you to abstract over return types, offering enhanced code flexibility while maintaining the benefits of Rust's strong type system. This feature is particularly useful when designing functions that return iterators or other complex types, enabling you to encapsulate implementation details while exposing only the behavior the caller needs.

Rust uses **static dispatch** to resolve the concrete return type of a function during compile time. This resolution process, known as **monomorphization**, involves the compiler generating specific implementations of functions or types for each concrete type used in your program. This ensures that Rust's zero-cost abstractions deliver optimal performance without runtime overhead.

In this task, you will implement a function that filters even numbers from a slice and returns an iterator over the filtered results. This approach is efficient and leverages Rust's iterator combinators.

## Your Task

Write a function named `filter_even_numbers` that:

1. Takes a slice of `i32` as input.
2. Returns an iterator that filters and yields only the even numbers.

### Requirements

1. The function must accept a slice of integers (`&[i32]`).
2. It must return an `impl Iterator<Item = &i32>`.
3. Use the `filter` method to include only even numbers.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `.iter()` method to create an iterator over the slice.
- Use the `filter()` method to filter out odd numbers.
- The return value of a `filter()` method is an iterator.
- `%` can determine whether a number is even.

</details>
