In Rust, ownership is a core concept that ensures memory safety. However, there are scenarios where multiple parts of your program need shared access to the same data. The `Rc<T>` type, which stands for "Reference Counted," is a smart pointer that provides shared ownership of immutable data. It keeps track of the number of references to the data and automatically deallocates it when there are no more references.

`Rc<T>` is primarily used in single-threaded scenarios. For multi-threaded programs, `Arc<T>` (atomic reference counting) should be used instead.

## Your Task

In this challenge, you will implement a function `clone_and_append` that:

1. Accepts an `Rc<Vec<T>>` where `T` implements the `Clone` trait.
2. Clones the original `Rc<Vec<T>>` and appends a new value to the cloned vector.
3. Returns the modified `Rc<Vec<T>>`.

This function demonstrates the ability to clone and manipulate data while keeping the original data intact for other owners.

## Requirements

### Constraints

1. Use `Rc<T>` to create shared ownership of the vector.
2. Ensure that the original vector is not modified.
3. The function must be generic over any type `T` that implements the `Clone` trait.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use `Rc::clone(&original)` to create a new reference to the original data.
- To modify the vector, you may need to dereference the `Rc` to access the underlying vector.
- Remember, `Rc<T>` only allows immutable access to the data it wraps. To modify the data, you will need to create a new instance.
- Review the methods available on `Vec<T>` to append items.

</details>
