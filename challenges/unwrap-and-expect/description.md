Rust provides two methods for handling unrecoverable errors with the `Result` and `Option` types: `unwrap` and `expect`. These methods extract the inner value but terminate the program if the operation fails. They are particularly useful for quick prototyping or situations where an error is truly unrecoverable.

In this challenge, you will interact with file operations to demonstrate the use of `unwrap` and `expect`. Instead of directly working with `Result` or `Option`, you will use standard library functions that return these types and handle their results using `unwrap` and `expect`.

## Your Task

Implement the following functions:

1. **`read_file_to_string`**: This function takes a file path as input, attempts to read its contents, and returns the contents as a `String`. Use `expect` to handle any file I/O errors with a custom error message.
2. **`get_env_variable`**: This function retrieves the value of an environment variable by name. Use `unwrap` to panic if the variable is not set.

### Requirements

1. `read_file_to_string(path: &str) -> String`:

   - Use `std::fs::read_to_string` to read the file.
   - If the file cannot be read, panic with an error message like `"Failed to read file: <path>"`.

2. `get_env_variable(key: &str) -> String`:
   - Use `std::env::var` to get the environment variable's value.
   - If the environment variable is not set, panic with a default `unwrap` message.

### Notes

- `expect` provides a way to add context to your panics, which can help with debugging.
- `unwrap` is more concise but less descriptive when errors occur.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Use `std::fs::read_to_string(path)` to read the contents of a file. It returns a `Result<String, std::io::Error>`.
- Use `std::env::var(key)` to retrieve an environment variable. It returns a `Result<String, std::env::VarError>`.
- Use `.expect()` to add a custom error message when handling a `Result`.
- Use `.unwrap()` for a quick, less descriptive error handling method.

</details>
