The `Default` trait is a powerful tool for reducing code repetition and managing complex configurations. When dealing with structs that have many fields, manually specifying every value can be tedious and error-prone. The `Default` trait, combined with the `..Default::default()` syntax, provides an elegant solution to this problem.

## Why Use Default?

Consider a scenario where you have a configuration struct with 8 fields, but you only want to customize 1 or 2 of them. Without `Default`, you would need to write:

```rust
let config = AppConfig {
    theme: String::from("Dark"),
    notifications_enabled: true,
    max_users: 100,
    auto_save: true,
    cache_size_mb: 512,
    log_level: String::from("INFO"),
    retry_attempts: 3,
    timeout_seconds: 30,
};
```

With `Default`, you can simply write:

```rust
let config = AppConfig {
    theme: String::from("Dark"),
    ..Default::default()
};
```

This not only reduces code repetition but also makes your code more maintainable and less prone to errors.

## Your Task

In this challenge, you will implement the `Default` trait for an `AppConfig` struct that has several configuration options. The struct should have the following fields and default values:

- `theme` (String): Default value `"Light"`
- `notifications_enabled` (bool): Default value `true`
- `max_users` (u32): Default value `100`
- `auto_save` (bool): Default value `true`
- `cache_size_mb` (u32): Default value `512`
- `log_level` (String): Default value `"INFO"`
- `retry_attempts` (u32): Default value `3`
- `timeout_seconds` (u32): Default value `30`

Your task is to:

1. Define the `AppConfig` struct with all specified fields
2. Implement the `Default` trait manually

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use `String::from()` for string values in the default implementation
- Remember that `..Default::default()` must come last in struct initialization
- Consider how this pattern would scale with even more configuration options
- Think about maintainability when default values need to change

</details>
