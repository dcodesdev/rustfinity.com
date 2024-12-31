Rust's `Option<T>` is a powerful type that represents a value that might or might not be present. It is often used to safely handle cases where a value could be missing or invalid.

The `Option<T>` enum has two variants:

- `Some(T)` which contains a value of type `T`.
- `None` which signifies the absence of a value.

## Your Task

In this challenge, you will implement a function `find_first_even` that takes a list of integers and returns the first even number in the list wrapped in `Some`. If no even number is present, the function should return `None`.

For example:

- If the input list is `[1, 3, 5, 8]`, the function should return `Some(8)`.
- If the input list is `[1, 3, 5]`, the function should return `None`.

Your task is to implement the function so it correctly handles any list of integers.

### Requirements

- If the list contains at least one even number, return the first even number wrapped in `Some`.
- If the list contains no even numbers, return `None`.
- You should use the `Option<T>` type effectively to solve this challenge.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use Rust's iterators to traverse the list and find the first even number.
- The `filter` method can be useful to narrow down the list to even numbers.
- Once you have a filtered iterator, use methods like `next` to get the first value.
- Remember, the `map` method can help you convert values within an `Option`.

</details>
