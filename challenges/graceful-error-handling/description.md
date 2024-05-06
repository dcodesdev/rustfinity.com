One of the most loved features of Rust is the way it lets you handle errors. The `Result` type is a powerful tool that allows you to handle errors in a way that is both safe and expressive. In this challenge, you will be working with the `Result` type to handle errors in a graceful way.

## Task

In this challenges, you're given a function, `parse_percentage(input: &str) -> Result<u8, String>` that takes a string as input and returns a `Result` type. The function should parse the input string as a percentage and return the percentage as a `u8` if the input is valid. If the input is invalid, the function should return an error message as a `String`.

Parsing from a string to a number can fail for many reasons. For example, the input string may not be a valid number, or it may be a valid number but not a valid percentage. Your task is to handle these errors gracefully and return an error message that explains what went wrong.

Complete the function, if the function returns an `Ok` value, print the value to the console. If the function returns an `Err` value, print the error message to the console.

## Requirements

- If the parse was successful, the function should return the success variant of `Result` with the percentage as a `u8`.
- If the number was out of range (not between 0 and 100), the function should return the error with `String` "Percentage out of range".
- If the string was not a valid number, the function should return the error with `String` "Invalid input".
