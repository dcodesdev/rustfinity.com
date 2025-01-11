In this challenge, you will harness the power of Rust macros to generate repetitive code. Your task is to create a macro that implements a trait called `DefaultValue` for multiple types, each with a specific "default" value. The `DefaultValue` trait provides a method to return a predefined value for a type.

Using a macro, you will generate implementations of this trait for various types, such as `f64`, `i32`, `u8`, and more.

For example:

- `impl DefaultValue for f64` should return `0.0` when `default_value` is called.
- `impl DefaultValue for u32` should return `2147483647` when `default_value` is called.
- `impl DefaultValue for u8` should return `127` when `default_value` is called.

### Your Task

1. Create a trait called `DefaultValue` with a single function `default_value() -> Self`.
2. Write a macro named `default_value_impl` that:
   - Accepts a type and its corresponding default value.
   - Automatically generates an implementation of the `DefaultValue` trait for that type.
3. Use the macro to implement the `DefaultValue` trait for at least the following types:
   - `f64` with `0.0`
   - `f32` with `0.0`
   - `u32` with `2147483647`
   - `u8` with `127`
   - `i32` with `0`
   - `u16` with `32767`
   - `i16` with `0`
   - `i8` with `0`

### Requirements

1. The macro should generate a correct implementation of the `DefaultValue` trait.
2. Ensure that each implementation uses the correct "default" value for its type.
3. Test your implementation to verify that calling `default_value` on each type returns the correct value.

### Hints

<details>
<summary>Click here to reveal hints</summary>

- Use `macro_rules!` to define the macro.
- The syntax for implementing a trait for a type is:
  ```rust
  impl TraitName for TypeName {
      fn function_name() -> ReturnType {
          // Implementation
      }
  }
  ```
- Use `$type` for the type and `$value` for the corresponding value in the macro.
- You can test your implementation by calling `<Type as DefaultValue>::default_value()`.

</details>
