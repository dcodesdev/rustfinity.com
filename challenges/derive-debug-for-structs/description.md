In Rust, the `Debug` trait allows you to format your types using the `{:?}` formatter, making it easier to inspect and debug your code. Instead of manually implementing the `Debug` trait for every struct, Rust provides the convenient `#[derive(Debug)]` attribute to automatically generate a `Debug` implementation.

For this challenge, you will work with structs that represent different types of data. Your task is to implement the `Debug` trait using the `derive` macro and demonstrate its functionality with sample structs.

## Your Task

1. Define several structs representing various entities (e.g., `Person`, `Point`, `Rectangle`).
2. Derive the `Debug` trait for each struct using `#[derive(Debug)]`.

### Requirements

1. Use the `#[derive(Debug)]` attribute for all structs.
2. Define at least three structs, each with different fields and types.
