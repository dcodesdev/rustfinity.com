Iterators are a powerful abstraction in Rust, enabling developers to process sequences of data in a functional, efficient, and composable way. They are the foundation of many collection-related operations and provide a clean and expressive alternative to traditional loops.

In this challenge, you will implement two functions that utilize iterators to perform specific operations:

1. Filtering out even numbers from a sequence of integers.
2. Converting strings to uppercase from a sequence of string slices.

Using Rust's iterator methods such as `filter`, `map`, and `collect`, you'll create concise, functional implementations that demonstrate the elegance and power of iterators.

## Your Task

Write the following two functions:

1. **`filter_even_numbers`**:  
   Takes an iterator of integers and filters out even numbers, returning a vector of the remaining odd numbers.

2. **`uppercase_strings`**:  
   Takes an iterator of string slices and converts each string to uppercase, returning a vector of the transformed strings.

Read the `main()` function carefully to understand how the functions will be used and what is expected from them.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `filter` method for filtering elements based on a condition.
- Make sure you dereference the values when using them in the closure. For example:

  ```rust
  .filter(|&x| x % 2 != 0)
  ```

  Or

  ```rust
  .filter(|x| *x % 2 != 0)
  ```

- Use the `collect` method to gather results from an iterator into a vector.
- Use the `map` method for transforming elements in an iterator.
- Use the `to_uppercase` method to convert a string to uppercase.
- Define lifetimes for the input and output types in the function signature.

</details>
