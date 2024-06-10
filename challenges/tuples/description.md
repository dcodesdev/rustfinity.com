Tuples are a simple and versatile data structure in Rust that allow you to **group multiple values of different types into a single compound value.** They are particularly useful for returning multiple values from a function.

**Tuples** can return multiple values of different types, which is not possible with arrays or slices. For example a tuple could be `(i32, f64, String)` which contains an integer, a float, and a string.

In this challenge, you will implement a function that takes three arguments of different types and returns them as a tuple.

## Your task

You need to implement the function `create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String)` that takes an **integer `i32`**, a **float `f64`**, and a **string slice `&str`** as input and returns them as a tuple. The string slice should be converted into a `String` type.

- The `create_tuple` function should return a tuple containing the three input values **in order**.
- The string slice input should be converted into a `String` before returning.

## Example

```rust
let result = create_tuple(42, 3.14, "hello");
assert_eq!(result, (42, 3.14, String::from("hello")));
```

## Hints

- Use parentheses `()` to define and return the tuple.
- Remember to convert the **string slice `&str`** to String using `String::from()` or the `.to_string()` method.
