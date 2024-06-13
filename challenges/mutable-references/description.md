Rust's ownership model ensures memory safety without needing a garbage collector. In the previous challenge, you learned about **immutable references**. Now, let's dive into **mutable references**.

## Mutable References

**Mutable references** allow you to **modify** the value you are **borrowing**. **You can only have one mutable reference to a particular piece of data in a particular scope.** This prevents data races at compile time.

### Example

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s); // borrow s as mutable
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Challenge

Create a function `append_suffix` that takes a mutable reference to a `String` and appends a given suffix to it.

## Requirements

- The `append_suffix` function should take **a mutable reference** to the input `String` and append the given suffix to it.

## Example

```rust
let mut s2 = String::from("hello");
append_suffix(&mut s2, " world");
assert_eq!(s2, "hello world");
```

## Hints

- The `String` type in Rust has methods like `push_str` which can be useful for modifying strings.
