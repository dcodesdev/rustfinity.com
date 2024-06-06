## Vectors

Vectors in Rust are a collection type that can store multiple values of the same type. They are similar to **arrays in JavaScript** and **lists in Python** but with some additional features, they are stored on the heap and they can grow or shrink in size.

In this challenge you're asked to implement two functions that calculate the **median and mode** of a list of integers.

Read the definitions of each **Median** and **Mode** below to understand what you need to implement.

### Median

The median is the middle value of a list when it is ordered by size. If the list has an even number of elements, the median is the average of the two middle numbers.

Here's an example:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
let median = median(&mut numbers);
assert_eq!(median, 5.0);
```

Or if the number of elements is even:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let median = median(&mut numbers);
assert_eq!(median, 5.5);
```

You can implement the function in the following steps:

- Sort the list of numbers.
- If the number of elements is odd, return the middle element.
- If the number of elements is even, return the average of the two middle elements.

### Mode

The mode is the number that appears most frequently in a list, if there are multiple numbers that appear the same number of times, return all of them in a vector.

Here's an example:

```rust
let numbers = vec![1, 1, 1, 1, 2]
let mode = mode(&numbers);
assert_eq!(mode, vec![1]);
```

The number `1` appears four times, which is more than any other number in the list.

> **Important**: You must sort the list of numbers before returning the modes list.

### Hints

- You can use the `sort` method to sort a vector.
- You can use the `entry` method to insert a value into a hash map if it doesn't exist or update it if it does.
- When looping through a vector, you can dereference the value using `&` to get the value instead of a reference to it. For example:

```rust
let numbers = vec![1, 2, 3, 4, 5];
for &number in &numbers {
    println!("{}", number);
}
```

- Store the maximum frequency and the number that appears most frequently in a `HashMap<i32, i32>` and update it as you iterate through the list of numbers, and finally return the list of numbers that appears most frequently.
