In Rust, the `Default` trait provides a convenient way to specify default values for types. This trait is commonly used when working with structs or enums, allowing you to create instances of these types without manually specifying all their fields.

The `Default` trait is part of the standard library and requires implementing a single method, `default`, which returns the default value for the type. Often, you can derive this trait for your type using `#[derive(Default)]`, but you can also implement it manually for custom behavior.

The syntax `..Default::default()` is a shorthand used to initialize the remaining fields of a struct with their default values when constructing an instance of the struct. This is particularly useful when only a few fields of the struct are explicitly set while the rest use their default values.

For example:

```rust
#[derive(Default)]
struct Config {
    debug: bool,
    retries: u32,
    timeout: u64,
}

let config = Config {
    debug: true,
    ..Default::default()
};
```

Here, the `debug` field is explicitly set, while `retries` and `timeout` are filled in with their default values.

## Your Task

In this challenge, you will implement the `Default` trait for a struct called `AppConfig`. The struct should represent a basic application configuration and have the following fields:

- `theme` (String): Default value `"Light"`.
- `notifications_enabled` (bool): Default value `true`.
- `max_users` (u32): Default value `100`.

Your task is to:

1. Define the `AppConfig` struct with the specified fields.
2. Implement the `Default` trait for the struct, providing the given default values.
3. Demonstrate how to use the `..Default::default()` syntax when creating instances of `AppConfig`.

## Requirements

### Constraints

- The struct must have the fields `theme`, `notifications_enabled`, and `max_users`.
- You must manually implement the `Default` trait for `AppConfig` rather than using the derive macro.
- Demonstrate how to use the `..Default::default()` syntax to set some fields explicitly while using defaults for others.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The `Default` trait requires a method `default` that returns `Self`.
- You can use the `String::from` method to create a `String` instance with a default value.
- When initializing a struct, use the `..Default::default()` syntax to fill in unspecified fields with their defaults.

</details>
