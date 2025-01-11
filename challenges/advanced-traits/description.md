You're already familiar with implementing traits in Rust, let's explore some special traits from the standard library - specifically those that allow operator overloading through the `std::ops` module.

One such trait is `Add`, which allows you to implement custom behavior for the `+` operator. This is particularly useful when working with types that have a natural addition operation, like measurements or mathematical types.

## Your Task

Let's implement custom addition between different units of measurement. You'll create a system where you can add meters to millimeters, with automatic unit conversion.

Your job is to implement a `Millimeters` struct and a `Meters` struct, then implement the `Add` trait to allow adding a `Meters` value to a `Millimeters` value. The result should be a new `Millimeters` value.

### Requirements

- Define two structs: `Millimeters` and `Meters`, each holding a single `u32` value
- Implement the `Add<Meters>` trait for `Millimeters` to handle addition with `Meters`
- Conversion rules:
  - 1 meter = 1000 millimeters
  - When adding, convert meters to millimeters first
  - Return the result as `Millimeters`

## Hints

If you're stuck, here are some hints to help you solve the challenge!

<details>
    <summary>Click here to reveal hints</summary>

- The `Add` trait requires you to specify:

  ```rust
  type Output = Millimeters;
  fn add(self, other: Meters) -> Self::Output;
  ```

- Remember that meters must be multiplied by 1000 to convert to millimeters
- Access tuple struct fields with `.0`

</details>
