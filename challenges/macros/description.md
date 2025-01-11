Macros in Rust are one of its most powerful features, enabling meta-programming and reducing repetitive code. A macro allows you to generate and include code fragments during compilation, making it an excellent tool for tasks like boilerplate elimination.

In this challenge, you will create a macro named `math_operations!` that evaluates a mathematical expression involving two numbers and formats the result as a string. The macro should take three arguments: the first number, the operator, and the second number, and return a formatted string of the operation and result.

For example:

- `math_operations!(4, "+", 2)` should return `"4 + 2 = 6"`.
- `math_operations!(10, "-", 3)` should return `"10 - 3 = 7"`.
- `math_operations!(6, "*", 4)` should return `"6 * 4 = 24"`.
- `math_operations!(15, "/", 3)` should return `"15 / 3 = 5"`.

### Your Task

Write the `math_operations!` macro to handle the following:

1. Perform basic mathematical operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
2. Return the formatted string showing the operation and the result.
3. Handle unsupported operators with a compile-time panic.
4. Panic if there is an attempt to divide by zero.

### Requirements

- The macro must take three arguments:
  - A numeric value (the first operand).
  - A string operator (`"+", "-", "*", "/"`).
  - Another numeric value (the second operand).
- If the operator is invalid, the macro should panic with a clear message: `"Unsupported operator: <operator>"`.
- If dividing by zero, the macro should panic with the message: `"Division by zero"`.

## Hints

If you're stuck, here are some tips to help you out:

<details>
  <summary>Click here to reveal hints</summary>

- Use a `match` statement inside the macro to evaluate the operator and apply the operation.
- The `panic!` macro is useful for handling invalid input at compile time.
- You can use the `expr` matcher for the operands and the operator in the macro's signature.
- Be sure to include zero-check logic when dividing to prevent runtime errors.

</details>
