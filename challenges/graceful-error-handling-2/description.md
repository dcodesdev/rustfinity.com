Rust’s error handling capabilities allow developers to create robust applications. In this challenge, you will define a custom error type as an enum, implement the `Error` trait, and use it to provide meaningful error messages. This is an extension of a previous challenge where errors were handled using plain strings. By using an enum, you can define specific error types, making your error handling more structured and maintainable.

## Your Task

You will implement a function, `parse_percentage`, which parses a string input and converts it into a `u8`. The function should return a `Result<u8, ParsePercentageError>`, where `ParsePercentageError` is a custom enum that represents the various types of errors that can occur.

### Requirements

1. Define an enum `ParsePercentageError` with the following variants:

   - `InvalidInput`: for inputs that cannot be parsed as numbers.
   - `OutOfRange`: for numbers that are not in the range 0-100.

2. Implement the `Error` trait for `ParsePercentageError`. Use the `std::error::Error` trait and provide human-readable descriptions for each error.

3. Update the `parse_percentage` function to:
   - Return `Ok(u8)` if the input is a valid percentage (between 0 and 100).
   - Return `Err(ParsePercentageError::OutOfRange)` if the number is out of range.
   - Return `Err(ParsePercentageError::InvalidInput)` if the input is not a valid number.

### Example

```rust
assert_eq!(parse_percentage("50"), Ok(50));
assert_eq!(parse_percentage("101"), Err(ParsePercentageError::OutOfRange));
assert_eq!(parse_percentage("abc"), Err(ParsePercentageError::InvalidInput));
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `std::fmt` module to implement `Display` for the error enum, which is required for the `Error` trait.
- Use pattern matching to handle parsing results and error scenarios.
- You can use `.parse::<u8>()` for string-to-integer conversion.

</details>
