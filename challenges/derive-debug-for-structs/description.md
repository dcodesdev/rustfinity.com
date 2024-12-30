In Rust, the `Debug` trait allows you to format your types using the `{:?}` formatter, making it easier to inspect and debug your code. Instead of manually implementing the `Debug` trait for every struct, Rust provides the convenient `#[derive(Debug)]` attribute to automatically generate a `Debug` implementation.

For this challenge, you will work with structs that represent different types of data. Your task is to implement the `Debug` trait using the `derive` macro and demonstrate its functionality with sample structs.

## Your Task

1. Define several structs representing various entities (e.g., `Person`, `Point`, `Rectangle`).
2. Derive the `Debug` trait for each struct using `#[derive(Debug)]`.
3. Write a function `debug_example()` that demonstrates printing the debug output of these structs.

### Requirements

1. Use the `#[derive(Debug)]` attribute for all structs.
2. Define at least three structs, each with different fields and types.
3. Use the `format!("{:?}", ...)` or `println!("{:?}", ...)` to output debug information.
4. Write test cases to ensure that the debug output contains the expected values for each struct.

## Hints

<details>
    <summary>Click here to reveal hints</summary>
    
- Use the `#[derive(Debug)]` attribute above your struct definitions to automatically implement the `Debug` trait.
- The `format!("{:?}", ...)` macro formats types that implement `Debug`.
- Make sure all fields in your structs also implement the `Debug` trait, as Rust requires this for deriving `Debug`.

</details>
