## Mutable Variables Challenge

Declaring and manipulating variables in programming is a fundamental concept that allows you to store and modify data. Variables in Rust are **immutable by default**, but you can make them mutable using the `mut` keyword.

In this challenge, you will declare and use **mutable variables in Rust**. You will be given a function where you need to declare variables, modify their values, and perform specific operations.

## Your task

In this challenge, you need to declare two mutable variables inside the function using the `let mut` keyword:

- `x` with an initial value of `5`
- `y` with an initial value of `10`

Then, perform the following operations:

1. Increment `x` by `10`
2. Multiply `y` by `2`

Finally, calculate the sum of `x` and `y` and return the result.

## Requirements

- Declare two mutable variables, `x` and `y`, and assign them initial values.
- Increment `x` by `10`.
- Multiply `y` by `2`.
- Calculate the sum of `x` and `y`.
- Return the calculated sum.

## Example

```rust
let result = manipulate_variables();
assert_eq!(result, 35);
```

## Hints

- Use the `let mut` keyword to declare **mutable variables**.
- Use the `+=` operator to increment a variable.
- Use the `\*=` operator to multiply a variable.
- Calculate the sum of `x` and `y` using the `+` operator.
- Either use a `return` statement or an **expression at the end without a semicolon** to return the result.
