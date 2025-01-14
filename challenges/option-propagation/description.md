In the previous challenge, you learned how to propagate errors using the `Result<T, E>` type for file handling. Rust also lets you use the `?` operator to propagate `Option<T>` values, but instead of propagating the error it will convert the `Option<T>` into a `None` value if it is `None`.

This challenge focuses on propagating `Option` values and handling scenarios where optional values might be missing.

## Your Task

Implement the function `find_and_multiply`:

1. It takes three parameters:
   - A vector of integers (`Vec<i32>`).
   - Two indices (`usize` values).
2. The function retrieves the integers at the two specified indices from the vector.
3. If both indices are valid, it returns the product of the two integers as an `Option<i32>`.
4. If any index is out of bounds, the function returns `None`.
5. Use the `?` operator to propagate `None` values.
