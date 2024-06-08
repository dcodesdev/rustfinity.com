## Mutable Variables Challenge

Declaring and manipulating variables in programming is a fundamental concept that allows you to store and modify data. Variables in Rust are **immutable by default**, but you can make them mutable using the `mut` keyword.

In this challenge, you will declare and use **mutable variables in Rust**. You will be given a function where you need to declare variables, modify their values, and perform specific operations.

## Your task

In this challenge, you need to declare one mutable variable inside the function using the `let mut` keyword:

- `text` with an initial value of `"hello"`
- Print it to the console using the `println!` macro.
- Mutate the variable to `"bye"`.
- Print the variable again to the console using the `println!` macro.
- Return the final value of the variable.

### Println! Macro

The `println!` macro is used to print text to the console. It is similar to the `println` function in other programming languages. The `println!` macro is used to print formatted text to the console.

```rust
println!("Hello, World!");
```

## Hints

- Use the `let mut` keyword to declare a mutable variable.
- Reassign the variable directly by using the `=` operator.
- Ensure to return the final value of the variable from the function.
- Use `println!` statements to debug and check the values during reassignment.
