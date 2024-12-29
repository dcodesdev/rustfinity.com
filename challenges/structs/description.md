Structs are one of Rust's fundamental building blocks for creating custom types. They allow you to group together related data under one type, making your programs easier to understand and maintain. In this challenge, you'll work with Rust structs by creating a simple structure to represent a `Person`.

A struct in Rust can have named fields with different data types. For example:

```rust
struct Book {
    name: String,
    author: String,
}
```

Once defined, you can create instances of the struct and access or modify its fields using dot notation.

---

## Your Task

Your task is to define a struct called `Person` with the following fields:

- `name`: A `String` representing the person's name.
- `age`: A `u8` representing the person's age.

You must also implement a function `is_adult` that takes a reference to a `Person` and returns `true` if the person's age is 18 or older, and `false` otherwise.

### Requirements

1. Define a struct called `Person` with the fields `name` and `age`.
2. Implement the function `is_adult` to determine if a person is an adult (18 or older).
3. Make sure the struct and it's fields are public.
   ```rust
   pub struct Book {
       pub name: String,
       pub author: String,
   }
   ```
   This will allow the tests to access the struct and its fields. Without this, the tests will fail.

---

## Example Test

Here's how your code will be tested:

```rust
let john = Person { name: "John".to_string(), age: 20 };
assert_eq!(is_adult(&john), true);

let emily = Person { name: "Emily".to_string(), age: 15 };
assert_eq!(is_adult(&emily), false);
```

---

## Hints

If you're having trouble, consider these hints:

<details>
  <summary>Click here to reveal hints</summary>

- Use `&` to pass references to avoid unnecessary copies.
- Use the `age` field directly in a comparison for the `is_adult` function.

</details>
