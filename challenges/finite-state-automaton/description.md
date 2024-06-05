In this challenge, you will implement a finite state automaton (FSA) to recognize a specific pattern in a sequence of characters. Finite state automatons are widely used in text processing, lexical analysis, and many other areas where pattern recognition is essential.

A finite state automaton is a mathematical model of computation used to design both computer programs and sequential logic circuits. It is a powerful tool for pattern matching as it consists of a finite number of states and transitions between these states based on input symbols.

## Your Task

You need to create an FSA that can recognize the pattern "ab\*c", where:

- 'a' is followed by zero or more 'b's and then followed by 'c'.

You will implement a function `recognize_pattern` that takes a string slice as input and returns a boolean indicating whether the input string matches the pattern.

## Requirements

- Implement the finite state automaton using Rust's enums and the `match` statement.
- The function should handle empty strings and any invalid input gracefully.
- Your FSA should consist of states and transitions implemented as Rust enums and functions.
- You must handle state transitions explicitly using the `match` statement.

## Example

```rust
let result = recognize_pattern("abbbc");
assert_eq!(result, true);

let result = recognize_pattern("ac");
assert_eq!(result, true);

let result = recognize_pattern("abbbd");
assert_eq!(result, false);

let result = recognize_pattern("");
assert_eq!(result, false);
```

> Did You Know?
>
> Finite state automatons have a wide range of applications outside computer science as well. For example, they are used in the design of digital circuits. In digital circuit design, an FSA can be used to create sequential circuits such as counters and communication protocol controllers. FSAs are also used in the field of linguistics to model the morphology of languages and in robotics to control the behavior of autonomous robots.

## Hints

- Think about how you can represent states and transitions using Rust enums.
- Carefully plan the transitions between states based on the input characters.
- Make sure to handle edge cases, such as an empty input string or invalid characters.
