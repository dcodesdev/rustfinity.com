Traits in Rust are a powerful feature that allow you to define shared behavior across types. In this challenge, you will delve into **advanced traits**, particularly focusing on implementing custom behavior for operator overloading using the `std::ops` module.

You are tasked with implementing a system that represents lengths in two units: millimeters and meters. To make these types work together seamlessly, you will implement the `Add` trait for `Millimeters` to allow adding a `Meters` value. This will enable conversions between the two units during addition.

## Your Task

Your job is to implement a `Millimeters` struct and a `Meters` struct. Then, implement the `Add` trait for `Millimeters` to allow adding a `Meters` value. The result should be a new `Millimeters` value, converting the `Meters` value to millimeters before adding.

### Requirements

- Define two structs: `Millimeters` and `Meters`, each holding a single `u32` value.
- Implement the `Add` trait for `Millimeters` to handle addition with `Meters`.
- Conversion logic:
  - 1 meter equals 1000 millimeters.
  - Add the millimeter equivalent of the `Meters` value to the `Millimeters` value.
- The output of the addition should be a new `Millimeters` instance with the correct total.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use the `std::ops::Add` trait, which requires an `add` method to be implemented.
- The `Add` trait takes an associated type `Output` that specifies the type of the result.
- You can access the inner value of a struct using `.0` if the struct has a single unnamed field.
- Structs in Rust can have type-specific implementations of traits.

</details>
