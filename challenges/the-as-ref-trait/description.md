Working with strings is a common requirement in real-world applications. Sometimes, you want a function that can take either a `String` or a `&str` as input without needing separate implementations. Rust's `AsRef` trait provides a simple way to borrow data as a reference in a generic manner, enabling functions to work with various types that can be represented as a reference to a common type.

The `AsRef` trait is a powerful standard library trait used for cheap conversions to a reference of another type. For example, it can be used to create a function that works with both owned (`String`) and borrowed (`&str`) string types. This approach is lightweight and avoids unnecessary allocations.

## Your Task

Implement a function `print_message` that accepts any type that implements the `AsRef<str>` trait. This function should:

- Accept a single argument of a generic type that implements `AsRef<str>`.
- Borrow the input as a string slice (`&str`) and print it to the console using `println!`.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The `AsRef` trait provides a `.as_ref()` method to borrow a reference.
- Both `String` and `&str` implement `AsRef<str>`, so they can be used interchangeably with this trait.

</details>
