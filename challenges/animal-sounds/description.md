Enums in Rust can have a combination of unit, tuple, and named field variants. This makes them incredibly versatile for representing complex and structured data.

In this challenge, you will create an `Animal` enum that demonstrates all three types of variants.

## Your Task

Create an enum `Animal` with the following variants:

1. **Unit Struct Variant**:

   - `Dog` — represents a generic dog.

2. **Tuple Struct Variant**:

   - `Cat(String)` — represents a cat, where the `String` contains the cat's name.

3. **Named Field Struct Variant**:
   - `Bird { species: String, can_fly: bool }` — represents a bird with its species and whether it can fly.

Write a function `describe_animal` that takes a reference to an `Animal` and returns a `String` description based on the variant:

- For `Dog`, return `"A friendly dog."`.
- For `Cat(name)`, return `"A cat named {name}."`.
- For `Bird { species, can_fly }`, return:
  - `"A {species} that can fly."` if `can_fly` is true.
  - `"A {species} that cannot fly."` if `can_fly` is false.

### Example

```rust
let dog = Animal::Dog;
assert_eq!(describe_animal(&dog), "A friendly dog.");

let cat = Animal::Cat("Whiskers".to_string());
assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

let bird = Animal::Bird { species: "Penguin".to_string(), can_fly: false };
assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
```

### Requirements

1. Define the `Animal` enum with the specified variants.
2. Implement the `describe_animal` function using a `match` expression to handle all variants.
3. Make sure the function takes a reference to the enum.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `match` statement to destructure tuple and named field variants.
- String formatting with `format!` makes it easy to include dynamic values in your description.
- `String` is a heap-allocated string; remember to handle it correctly when matching.

</details>
