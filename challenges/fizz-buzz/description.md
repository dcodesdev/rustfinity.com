Your task is to write a Rust program that implements the classic programming challenge known as **FizzBuzz**.

You are given a function `fizz_buzz(num: u32) -> String` that takes a single parameter `num` of type `u32` and returns a `String`.

Your job is to return a string based on the following rules:

- Return "Fizz" for numbers that are multiples of 3.
- Return "Buzz" for numbers that are multiples of 5.
- Return "FizzBuzz" for numbers that are multiples of both 3 and 5.
- Return the number itself for all other numbers.

## Example

Here's how the behavior of the function should look like:

```rust
assert_eq!(fizz_buzz(1), "1");
assert_eq!(fizz_buzz(2), "2");
assert_eq!(fizz_buzz(3), "Fizz");
assert_eq!(fizz_buzz(4), "4");
assert_eq!(fizz_buzz(5), "Buzz");
assert_eq!(fizz_buzz(6), "Fizz");
assert_eq!(fizz_buzz(7), "7");
assert_eq!(fizz_buzz(8), "8");
assert_eq!(fizz_buzz(9), "Fizz");
assert_eq!(fizz_buzz(10), "Buzz");
assert_eq!(fizz_buzz(11), "11");
assert_eq!(fizz_buzz(12), "Fizz");
assert_eq!(fizz_buzz(13), "13");
assert_eq!(fizz_buzz(14), "14");
assert_eq!(fizz_buzz(15), "FizzBuzz");
```

Good luck, and happy coding!
