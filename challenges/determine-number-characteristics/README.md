In this challenge, you will create a function that **determines the characteristics of a given number**. Specifically, the function will evaluate whether a number **is positive negative, or zero and also determine if it is even or odd**.

Rust's powerful control flow mechanisms, including `if`, `else if`, and `else` statements, make it easy to handle multiple conditions. By combining these conditions with logical operators, you can create a function that provides comprehensive information about the number.

## Your task

You are given a function `describe_number(n: i32) -> String` that takes an integer as input and returns a string description of the number's characteristics. The description should include whether the number is positive, negative, or zero and whether it is even or odd.

For example:

- "Positive even"
- "Positive odd"
- "Negative even"
- "Negative odd"
- "Zero"

Complete the function by implementing the necessary conditions and logical checks.

## Requirements

- If the number is positive and even, return "Positive even".
- If the number is positive and odd, return "Positive odd".
- If the number is negative and even, return "Negative even".
- If the number is negative and odd, return "Negative odd".
- If the number is zero, return "Zero".

## Example

```rust
let result = describe_number(10);
assert_eq!(result, "Positive even");

let result = describe_number(-3);
assert_eq!(result, "Negative odd");

let result = describe_number(0);
assert_eq!(result, "Zero");
```

## Hints

- Remember that the **modulus operator `%`** can help determine if a number is **even or odd**.
- **Logical operators `&&` and `||`** will be useful for **combining conditions**.
- Consider the **order** of your conditions to ensure the correct output.
