Closures in Rust are anonymous functions that can **capture variables from their environment.** They are similar to functions but have some unique properties that make them powerful and flexible. Closures are often used for short, simple operations and can be defined in a very concise way.

## Your Task

Your task is to complete the implementation of the following closures:

- `add_closure`: This closure should return the **sum of two integers**.
- `subtract_closure`: This closure should return the **difference between two integers**.
- `multiply_closure`: This closure should return the **product of two integers**.

### Requirements

- Each closure should take two parameters of type `i32`.
- Each closure should return a result of type `i32`.

## Hints

Here are some hints to help you solve the exercise:

<details>
    <summary>Click here to reveal hints</summary>

- Use the `|a, b|` syntax to define the parameters of the closure.
- Since the type is already specified, it can be inferred, so you don't need to specify `a` and `b`.

</details>
