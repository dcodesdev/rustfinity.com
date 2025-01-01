What we covered in the previous challenges were **recoverable errors**, meaning they can be handled during runtime. However, in Rust we have **unrecoverable errors** that are so severe that the program cannot safely continue and it will shut down completely.

You can trigger an unrecoverable error using the `panic!` macro, for example: `panic!("This is an unrecoverable error!")`.

In this challenge, you will write a function that retrieves the value of an environment variable named `DATABASE_URL` if the environment variable is not set, you will exit the program using `panic!`, it makes sense to use `panic!` instead of `Result<T, E>` because the program will not be able to run database queries if this value isn't available.

Just to make it more interesting, there is another requirement, your function must validate that the value starts with `postgresql://`, this is not a full proof validation for real life applications, but we'll keep it simple for this challenge.

## Your Task

Write a function, `get_database_url`, that retrieves the value of the `DATABASE_URL` environment variable and validates it. The function should behave as follows:

1. If the `DATABASE_URL` variable is set and starts with `postgresql://`, return its value as a `String`.
2. If the `DATABASE_URL` variable is not set, the function should terminate the program by calling `panic!` with the exact message `"DATABASE_URL environment variable is not set."`.
3. If the `DATABASE_URL` variable is set but does not start with `postgresql://`, the function should terminate the program by calling `panic!` with the message `"DATABASE_URL must start with 'postgresql://'"`.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Use the `std::env::var()` function to retrieve the value of an environment variable. e.g. `std::env::var("DATABASE_URL")`.
- Use the `starts_with` method to check if a string starts with a specific prefix. e.g. `my_string.starts_with("prefix")`.

</details>
