Sometimes you run operations that might fail, and you don't care about the specific error details. You just want to know whether the operation succeeded or failed. Rust’s `Result<T, E>` can be converted into an `Option<T>` to represent success (`Some`) or failure (`None`), discarding the error details.

This challenge builds on the concept of handling `Result` and converting it to `Option`. You will write a function that reads the entire content of a file and returns it as an `Option<String>`.

## Your Task

Implement the function `read_file`:

1. It takes a file path (`&str`) as input.
2. Attempts to open the file and read its entire content as a `String`.
3. If the operation is successful, return `Some(String)` containing the file content.
4. If any error occurs (e.g., file not found, permission issues, etc.), return `None`.
5. Use the `.ok()` method to convert `Result` into `Option` and use the `?` operator to easily propagate errors if required.
