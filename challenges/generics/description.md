Generics in Rust allow you to write code that works with different types without duplicating code for each specific type. They are commonly used in functions, structs, enums, and methods to make your code more reusable and flexible.

For this challenge, you will implement a generic struct `ItemContainer` that can store any type of item and provide methods to retrieve and manipulate the item.

## Your Task

1. Define a generic struct `ItemContainer<T>` with a single field `item: T`.
2. Implement the following methods for `ItemContainer`:
   - `get_item`: Returns a reference to the item.
   - `replace_item`: Replaces the item with a new one and returns the old item.

## Requirements

- Use generics for the struct and methods.
- Test `ItemContainer` with different types, such as integers, strings, and custom structs.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Define the struct as `ItemContainer<T>`.
- Use `&self` for the `get_item` method and `self` for the `replace_item` method.
- Use the `std::mem::replace` function to swap items efficiently.
</details>
