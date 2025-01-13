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

You are given a function `calculate_and_modify` that violates Rust's ownership rules. Your task is to fix this function by **moving exactly one line of code to a different position**.

**Don't just try to make the tests pass - focus on understanding which single line needs to be moved and why!**

## Hints

If you're stuck, here are some hints that can help you:

<details>
  <summary>Click to reveal hints</summary>

In order to understand the problem, we need to undertand the signature of the `push_str` method. The method signature is as follows:

```rust
fn push_str(&mut self, string: &str)
```

What's important to note is that, the method takes a mutable reference to `self`

Remember ownership rules? **You can have either one mutable reference or any number of immutable references, but not both simultaneously.**

In this case we have an immutable reference `s2` and the `push_str` method is trying to get a mutable reference to `s`, hence the violation of ownership rules.

The solution would be, moving the line that borrows `s` as immutable reference after the `push_str` method call. This way, we are not borrowing `s` as immutable reference when the mutable reference is still in scope, and the compiler can assure that the value hasn't changed.

```rust
pub fn calculate_and_modify() -> (String, usize) {
    let mut s = String::from("hello");
    let length = s.len();

    s.push_str(", world");

    let s2 = &s;
    println!("{}", s2);

    (s, length)
}
```

</details>
