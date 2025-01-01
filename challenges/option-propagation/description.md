In the previous challenge, you learned how to propagate errors using the `Result<T, E>` type for file handling. Rust also provides the `Option<T>` type for scenarios where a value might or might not exist. Just like `Result`, you can propagate `Option` values using the `?` operator.

This challenge focuses on propagating `Option` values and handling scenarios where optional values might be missing.

## Your Task

Implement the function `find_and_multiply`:

1. It takes three parameters:
   - A vector of integers (`Vec<i32>`).
   - Two indices (`usize` values).
2. The function retrieves the integers at the two specified indices from the vector.
3. If both indices are valid, it returns the product of the two integers as an `Option<i32>`.
4. If any index is out of bounds, the function returns `None`.

### Requirements

- Use the `Option<T>` type to handle cases where a value might be missing.
- Use the `?` operator to propagate `None` values.
- Ensure the function is concise and idiomatic.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- The `Vec::get` method returns an `Option<&T>` that can be used to safely access elements.
- The `?` operator can be used with `Option` just like with `Result`.
- Remember to handle the `None` case gracefully by propagating it.

</details>
