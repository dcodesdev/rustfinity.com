Associated types in Rust allow you to define traits with a type placeholder that implementors of the trait can specify. This enables more flexible and type-safe abstractions.

For example, instead of passing types as generic parameters every time, you can use associated types to simplify the interface. Associated types are particularly useful when working with collections, iterators, or data processing pipelines.

In this challenge, you'll define a trait with an associated type and implement it for a struct. The goal is to create a structure for managing a simple Key-Value store where keys and values can have different types specified by the trait implementation.

## Your Task

Define a trait `KeyValueStore` with an associated type `Key` and `Value`. Implement this trait for the struct `InMemoryStore`. The implementation should allow storing and retrieving key-value pairs of the specified types.

### Requirements

1. Define a trait `KeyValueStore` with:

   - An associated type `Key`.
   - An associated type `Value`.
   - Methods:
     - `set(&mut self, key: Self::Key, value: Self::Value)` to add a key-value pair.
     - `get(&self, key: &Self::Key) -> Option<&Self::Value>` to retrieve a value by key.

2. Create a struct `InMemoryStore` that uses a `HashMap` to store key-value pairs. Implement the `KeyValueStore` trait for this struct.

3. Your implementation should ensure type safety by enforcing the correct types for keys and values.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- You can use the `HashMap` from Rust's standard library as the internal storage for your `InMemoryStore`.
- Define associated types in the `KeyValueStore` trait like this:
  ```rust
  type Key;
  type Value;
  ```
- Remember to use the fully qualified syntax for methods in traits when needed, such as `<YourType as YourTrait>::AssociatedType`.
- You may need to derive traits like `Eq`, `Hash`, and `Clone` for your keys and values to work seamlessly with `HashMap`.

</details>
