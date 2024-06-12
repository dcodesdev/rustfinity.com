Closures in Rust are anonymous functions that can **capture variables from their environment.** They are similar to functions but have some unique properties that make them powerful and flexible. Closures are often used for short, simple operations and can be defined in a very concise way.

## Understanding Closures

A closure is defined using a pair of vertical bars `|` that enclose the parameters, followed by the closure body. Here's a simple example of a closure that adds two numbers:

```rust
let add = |a: i32, b: i32| a + b;
```

In this example, add is a closure that takes two parameters, `a` and `b`, and returns their **sum**. **You can call this closure just like a function**:

```rust
let result = add(2, 3); // result is 5
```

Closures can **capture variables** from their enclosing scope. For example:

```rust
let x = 2;
let add_x = |a: i32| a + x;
let result = add_x(3); // result is 5
```

In this case, the closure `add_x` captures the variable `x` from the surrounding scope and adds it to its parameter `a`.

## Your task

Your task is to complete the implementation of the following closures:

- `add_closure`: This closure should return the **sum of two integers**.
- `subtract_closure`: This closure should return the **difference between two integers**.
- `multiply_closure`: This closure should return the **product of two integers**.

## Requirements

- Each closure should take two parameters of type `i32`.
- Each closure should return a result of type `i32`.

## Example

```rust
assert_eq!(add_closure(3, 4), 7);
assert_eq!(subtract_closure(10, 4), 6);
assert_eq!(multiply_closure(3, 5), 15);
```

## Hints

- Remember to use the `let` keyword to define closures.
- Use the `|a, b|` syntax to define the parameters of the closure.
