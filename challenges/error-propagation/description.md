Error propagation is an essential concept in Rust that allows you to handle errors in a clean and concise way. When working with file I/O, errors can arise from missing files, permission issues, or unexpected file contents. By using the `?` operator, you can propagate errors and handle them at higher levels of your application.

In this challenge, you will read integers from a file, compute their sum, and gracefully handle errors that might occur.

## Your Task

Implement the function `sum_integers_from_file`:

- This function takes a file path as input.
- Reads the file line by line, assuming each line contains a single integer.
- Computes and returns the sum of all integers as a `Result<i32, String>`.
  - If the file cannot be opened, return an error message like `"Error reading file: <reason>"`.
  - If a line contains invalid data, return an error message like `"Invalid integer in file: <line>"`.

### Requirements

1. Use `std::fs::File` and `std::io::{self, BufRead}` to read the file.
2. Handle errors cleanly and propagate them using the `?` operator.
3. Return clear and meaningful error messages.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- Use `std::fs::File::open` to open the file.
- Use `io::BufReader::new` to read lines from the file.
- Convert strings to integers with the `str::parse` method.
- The `?` operator can propagate errors to the caller automatically.

</details>
