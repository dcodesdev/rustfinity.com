You are given a string `s`. Your task is to count the number of characters in the string and return the **total amount of characters** in the string with the type `u32`.

## Example

```rust
let s = "hello";
let result = character_counting_string(s);
assert_eq!(result, 5);
```

## Hints

- You can use the `chars()` method to get an iterator over the characters in the string.
- You can use the `count()` method to count the number of elements in the iterator.

> The `count()` method returns a `usize` which is the number of elements in the iterator. In the challenge you are asked to return a `u32`, you can use the `as` keyword to convert the `usize` to a `u32`. For example, `let count_u32 = count as u32;`
