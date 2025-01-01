Rust provides two primary methods for dealing with errors: recoverable errors and unrecoverable errors. For unrecoverable errors, the `panic!` macro is used to terminate the program immediately. This is helpful when an error occurs that makes it impossible or unsafe to continue the program.

In this challenge, you will write a function that retrieves the value of an environment variable named `DATABASE_URL`. Additionally, your function must validate that the value starts with `postgresql://`. If the environment variable is not set or does not start with the required prefix, your function should terminate the program using `panic!` with an appropriate error message.

This challenge demonstrates how to handle unrecoverable errors while ensuring the correctness of critical input values.

## Your Task

Write a function, `get_database_url`, that retrieves the value of the `DATABASE_URL` environment variable and validates it. The function should behave as follows:

1. If the `DATABASE_URL` variable is set and starts with `postgresql://`, return its value as a `String`.
2. If the `DATABASE_URL` variable is not set, the function should terminate the program by calling `panic!` with the message `"DATABASE_URL environment variable is not set."`.
3. If the `DATABASE_URL` variable is set but does not start with `postgresql://`, the function should terminate the program by calling `panic!` with the message `"DATABASE_URL must start with 'postgresql://'"`.

You will use the `std::env` module to work with environment variables and the `starts_with` method to validate the prefix.

### Requirements

- Use `panic!` for any unrecoverable errors (missing variable or invalid format).
- Provide clear and descriptive error messages when calling `panic!`.
- Use the `std::env::var` function to access environment variables.
- Ensure the validation logic checks if the string starts with `postgresql://`.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Use the `std::env::var` function to retrieve the value of an environment variable.
- If the environment variable is not set, `std::env::var` will return a `Result` with an `Err` variant.
- The `starts_with` method can be used to check if a string begins with a specific prefix.
- The `panic!` macro can take a custom error message, such as `"DATABASE_URL must start with 'postgresql://'."`.
- Make sure to test your function with both valid and invalid environment variable values.

</details>
