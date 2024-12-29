Constructors in Rust are special methods that are often used to create instances of a struct. By convention, constructors are implemented as associated functions named `new`. They encapsulate the initialization of the struct, ensuring that instances are always created in a valid state.

In this challenge, you will implement a constructor for a struct that represents a **Book**. A `Book` will have the following fields:

- `title`: A string that represents the book's title.
- `author`: A string that represents the book's author.
- `year`: An integer that represents the year the book was published.

The constructor function, `Book::new`, should take three parameters (`title`, `author`, and `year`) and return a fully initialized `Book`.

## Your Task

Implement the `Book` struct and its constructor, ensuring that it correctly initializes all fields.

## Requirements

1. The struct must be named `Book`.
2. Implement a constructor function `Book::new` that:

   - Takes three arguments: `title` (string slice), `author` (string slice), and `year` (integer).
   - Returns a `Book` instance with the specified values.

3. Ensure the constructor is idiomatic and handles the provided input types correctly.

## Example Test

```rust
let book = Book::new("The Rust Programming Language", "Steve Klabnik and Carol Nichols", 2019);

assert_eq!(book.title, "The Rust Programming Language");
assert_eq!(book.author, "Steve Klabnik and Carol Nichols");
assert_eq!(book.year, 2019);
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `String` type for storing text fields (`title` and `author`) inside the struct.
- Use `to_string()` or `String::from()` to convert string slices into `String` values.
- Remember to use `pub` for fields or provide accessor methods if the fields should be accessible outside the struct.

</details>
