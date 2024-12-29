Rust provides a special kind of struct known as a _tuple struct_. Unlike a regular struct with named fields, a tuple struct groups fields together without names, making it useful for lightweight structures where naming each field isn't necessary.

A tuple struct looks like this:

```rust
struct Point(i32, i32, i32);
```

You can create an instance of a tuple struct just like you would with a tuple and access its fields using dot notation.

## Your Task

Your task is to define a tuple struct called `Rectangle` with two fields:

- The width of the rectangle as a `u32`.
- The height of the rectangle as a `u32`.

Then, implement a function `area` that calculates and returns the area of the rectangle by multiplying its width and height.

### Requirements

1. Define a tuple struct `Rectangle(u32, u32)`.
2. Implement the function `area(rect: &Rectangle) -> u32`.

---

## Example Test

Here's how your code will be tested:

```rust
let rect = Rectangle(4, 5);
assert_eq!(area(&rect), 20);

let square = Rectangle(10, 10);
assert_eq!(area(&square), 100);
```

---

## Hints

If you get stuck, consider these hints:

<details>
    <summary>Click here to reveal hints</summary>

- Use dot notation to access the fields of the tuple struct (e.g., `rect.0` for width and `rect.1` for height).

</details>
