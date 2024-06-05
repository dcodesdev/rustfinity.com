In this challenge, you will demonstrate your understanding of **control flow** in Rust. The task involves finding the first **palindrome number** in a given range.

### What's a palindrome?

**A palindrome is a number that reads the same backward as forward**. This exercise will require you to iterate through the range, check each number to see if it is a palindrome, and return the first palindrome found. You can use any **control flow** construct to solve this problem.

Palindromes are fascinating numbers, and finding them within a range will require efficient control flow logic to ensure you identify the first one accurately.

## Your task

You need to write a function, `find_first_palindrome(start: i32, end: i32) -> Option<i32>`, that takes two **integer** arguments `start` and `end`. The function should return the first palindrome number within the range (inclusive). If there are no palindromes in the range, it should return `None`.

## Requirements

- The function should iterate through the numbers from `start` to `end`.
- It should check each number to determine if it is a **palindrome**.
- The function should **return the first palindrome found**, or `None` if no palindromes exist in the range.
- You must handle cases where `start` may be greater than `end`.

## Example

```rust
let result = find_first_palindrome(10, 30);
assert_eq!(result, Some(11)); // 11 is the first palindrome in the range

let result = find_first_palindrome(100, 105);
assert_eq!(result, Some(101)); // 101 is the first palindrome in the range

let result = find_first_palindrome(123, 130);
assert_eq!(result, None); // No palindromes in this range
```

## Did you know?

Did you know that **palindromes** are not just limited to numbers? They are found in **words, phrases, and even DNA sequences!** For example, the word **"racecar"** is a palindrome, as it reads the same backward and forward. **Palindromes** are fascinating in various fields, including mathematics, literature, and biology, where they often have unique properties and significance.

## Hints

- To check if a number is a palindrome, convert it to a **string** and compare it with its **reverse**.
- You can get the **reverse** by using the `rev()` method on a `char`.
- You can get the `char`s in a **string** by using the `chars()` method on a `String`
- Use a **loop**, iterator, or any other **control flow** construct to check each number in the range until a **palindrome** is found or the range is exhausted.
- Remember to handle the case **where start is greater than end** by swapping the values if necessary.
