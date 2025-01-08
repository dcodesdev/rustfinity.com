The `Box<T>` type in Rust is a smart pointer that allows heap allocation for values, enabling efficient management of large data or data with unknown sizes at compile time. Combining `Box<T>` with structs and the `Deref` trait expands its utility, enabling customized behavior when accessing heap-allocated values.

In this challenge, you'll work with a custom struct `Animal` and use `Box<T>` to manage its memory. You will also implement a function to access the struct's fields by dereferencing the boxed instance.

### The `Deref` Trait

The `Deref` trait enables customization of the behavior when a value is dereferenced using the `*` operator. Implementing `Deref` for a type allows it to mimic pointer-like behavior, which is useful when working with `Box<T>`.

### Your Task

1. Define a struct `Animal` with the following fields:

   - `name: String`
   - `age: u8`

2. Implement the `create_animal` function to return a `Box<Animal>` containing a new `Animal` instance.

3. Define another function, `access_animal`, that takes a `Box<Animal>` and returns a tuple `(String, u8)` representing the animal's name and age. Use dereferencing to access the fields.

4. Ensure your solution uses the `Deref` trait effectively to access the struct fields.

### Requirements

- Use `Box<T>` to allocate the `Animal` instance on the heap.
- Define the `Animal` struct with the specified fields.
- Implement both `create_animal` and `access_animal` functions.
- Use dereferencing to access the `Animal` fields in the `access_animal` function.

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- To create a boxed struct, use `Box::new(struct_instance)`.
- Use the `*` operator to dereference the box and access its value.
- For fields like `String`, cloning or borrowing may be necessary depending on the use case.
- Use tuple syntax `(field1, field2)` to return multiple values from a function.

</details>
