Declaring variables in programming is a fundamental concept that allows you to **store and manipulate data**. Variables are used to store values that can be accessed and modified throughout the program.

In Rust, variables are declared using the `let` keyword followed by the variable name and an optional type annotation.

In Rust, variables are **immutable by default**. This means that once a value is bound to a variable, it cannot be changed. This is a key feature of Rust that helps ensure safety and prevent bugs by avoiding unexpected changes to values.

## Your task

In this challenge, you will declare and use immutable variables in Rust. You will be given a function where you need to **declare variables** and use them to perform specific operations. The goal is to get comfortable with the concept of **immutability and understand how to use immutable variables effectively.**

Your task is to define two immutable variables inside the function using the `let` keyword:

- `width` with a value of `10`
- `height` with a value of `5`

Then, calculate the area of a rectangle using the formula `area = width * height` and return the calculated area.

## Requirements

- Declare two immutable variables, `width` and `height`, and assign them values.
- Calculate the area of the rectangle using the formula `width * height`.
- Return the calculated area.

## Hints

- Use the `let` keyword to declare variables.
- Remember that variables declared with `let` are immutable by default.
- You do not need to explicitly annotate the types (e.g., `let width: u32 = 10;`) for the variables.
- Use multiplication `*` to calculate the area.
