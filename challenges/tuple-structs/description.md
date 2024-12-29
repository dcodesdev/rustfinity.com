Rust provides a special kind of struct known as a _tuple struct_. Unlike a regular struct with named fields, a tuple struct groups fields together without names, making it useful for lightweight structures where naming each field isn't necessary.

A tuple struct looks like this:

```rust
struct Point(i32, i32, i32);
```

You can create an instance of a tuple struct just like you would with a tuple and access its fields using dot notation.

## Your Task

Your task is to define a tuple struct called `Rectangle` with two fields:

- The width of the rectangle as a `f32`.
- The height of the rectangle as a `f32`.

Then, implement a function `area` that calculates and returns the area of the rectangle by multiplying its width and height.

### Requirements

- The fields of the tuple struct must be public. Here's an example:
  ```rust
  pub struct Point(pub i32, pub i32, pub i32);
  ```

---

## Example Test

Here's how your code will be tested:

```rust
let rect = Rectangle(4.0, 5.0);
assert_eq!(area(&rect), 20.0);

let square = Rectangle(10.0, 10.0);
assert_eq!(area(&square), 100.0);
```

---

## Hints

If you get stuck, consider these hints:

<details>
    <summary>Click here to reveal hints</summary>

- Use dot notation to access the fields of the tuple struct (e.g., `rect.0` for width and `rect.1` for height).

</details>
