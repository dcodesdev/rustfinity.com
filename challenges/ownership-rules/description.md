Rust's ownership model ensures memory safety without needing a garbage collector. **Ownership rules are crucial for writing safe and efficient Rust code.** Sometimes, beginners may write code that violates these rules. **In this challenge, you will fix a piece of Rust code that has ownership rule violations.**

## Ownership Rules

Here are some important ownership rules to remember:

1. **Each value in Rust has a variable that's called its owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value will be dropped.**
4. **You can have either one mutable reference or any number of immutable references, but not both simultaneously.**

## Your task

Before starting to solve the challenge, try to run the code and see what error you'll get. This will help you understand the problem better.

The compile error tells us that we can't borrow `s` as mutable because it is also borrowed as immutable. If we borrow it as mutable, the value will change and the compiler can not guarantee that the immutable reference hasn't changed.

You are given a function `calculate_and_modify` that violates Rust's ownership rules. Your task is to fix this function by moving exactly one line of code to a different position. Don't just try to make the tests pass - focus on understanding which single line needs to be moved and why!

- The solution requires moving only one line to a different position - no other changes!
- The code should print the reference `s2` without any ownership rule violations
- The code should run the `push_str` method to mutate the string
- The code should return the modified string and its length

## Hints

- Look carefully at when you create the reference and when you use it
- Think about when the mutable borrow needs to happen in relation to the immutable reference
