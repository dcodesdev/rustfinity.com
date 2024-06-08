## Mutable Variables Challenge

Declaring and manipulating variables in programming is a fundamental concept that allows you to store and modify data. Variables in Rust are **immutable by default**, but you can make them mutable using the `mut` keyword.

In this challenge, you will declare and use **mutable variables in Rust**. You will be given a function where you need to declare variables, modify their values, and perform specific operations.

## Your task

In this challenge, you need to declare one mutable variable inside the function using the `let mut` keyword:

- `text` with an initial value of `"hello"`
- Print exactly `"Text is hello"` to the console using the `println!` macro.
- Mutate the variable to `"bye"`.
- Print exactly `"Text is bye"` to the console using the `println!` macro.
- Return the final value of the variable.

### Println! Macro

The `println!` macro is used to print text to the console. It is similar to the `println` function in other programming languages. The `println!` macro is used to print formatted text to the console.

```rust
let text = "hello";
println!("Text is {}", text);
```

The `{}` is a placeholder that will be replaced by the value of the variable `text`.

## Hints

- Use the `let mut` keyword to declare a mutable variable.
- Reassign the variable directly by using the `=` operator.
- Ensure to return the final value of the variable from the function.
