Rust provides a clear distinction between `Option<T>` for cases where a value might not exist and `Result<T, E>` for cases where additional error information is needed. In this challenge, you will safely retrieve the first element of a vector, returning it as a `Result` with appropriate error handling and validation.

This is useful in scenarios where you want to ensure a non-empty vector and validate the value of the first element.

## Your Task

Implement the function `get_first_element`:

1. Takes two parameters:
   - A vector of integers (`Vec<i32>`).
   - A minimum allowed value (`i32`).
2. Safely retrieves the first element using `Vec::first`, which returns an `Option<&T>`.
3. If the vector is empty, return a `Result` with the error message `"Vector is empty"`.
4. If the first element is less than the minimum allowed value, return a `Result` with the error message `"First element is below the minimum allowed value"`.
5. Otherwise, return `Ok(i32)` containing the first element.

### Requirements

- Use `Vec::first` to retrieve the first element of the vector.
- Use `.ok_or()` to convert the `Option` into a `Result`.
- Use `if` statements for validation logic.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- Use `Vec::first` to safely retrieve the first element as `Option<&T>`.
- Use `.ok_or()` to provide a meaningful error message when the vector is empty.
- Perform validation using `if` statements.

</details>
