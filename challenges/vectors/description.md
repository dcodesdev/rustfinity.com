Vectors are one of Rust's most commonly used collections. They are growable arrays that can hold multiple elements of the same type. They are defined in Rust using the `Vec<T>` type, where `T` is the type of elements the vector contains.

In this challenge, you will perform basic operations on a vector, such as adding, removing, and accessing elements. Understanding how to work with vectors is essential for building efficient and idiomatic Rust applications.

## Your Task

You are required to implement the following functions:

1. `add_elements`: This function takes a mutable reference to a vector of integers and a slice of integers. It appends all elements from the slice to the vector.
2. `remove_element`: This function takes a mutable reference to a vector of integers and an index. It removes the element at the given index if it exists.
3. `get_element`: This function takes a reference to a vector of integers and an index. It returns the element at the given index as an `Option<i32>`.

### Example

```rust
let mut vec = vec![1, 2, 3];
add_elements(&mut vec, &[4, 5]);
assert_eq!(vec, vec![1, 2, 3, 4, 5]);

remove_element(&mut vec, 2);
assert_eq!(vec, vec![1, 2, 4, 5]);

assert_eq!(get_element(&vec, 1), Some(2));
assert_eq!(get_element(&vec, 10), None);
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- You can use `vec.extend_from_slice(elements)` to add all elements from a slice to a vector.
- Use `Vec::remove` to remove an element by index.
- To safely access an element by index, use the `get` method on vectors.
- Keep in mind that `Vec::remove` will panic if the index is out of bounds, so handle it carefully in `remove_element`.
</details>
