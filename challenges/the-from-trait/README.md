The `From` trait is one of the most commonly used trait in the Rust programming language, **it is used for converting a value from one type to another.**

After you implement the `From` trait for a type, you can then use `into()` on the source type to convert this type to the target type.

Here's an example on a use case for the `From` trait:

```rust
fn main() {
    let f = Fahrenheit(32.0);
    let c: Celsius = f.into();
    println!("{}°F is {}°C", f.0, c.0);
}
```

This code works because the `Fahrenheit` struct implements the `From` trait for the `Celsius` struct. This allows us to convert a `Fahrenheit` instance to a `Celsius` instance using the `into()` method. The explicit type annotation `: Celsius` tells the compiler to turn it "into" a `Celsius` instance.

## Task

Your task is to implement the `From` trait for the following struct types:

```rust
struct Minutes(u32);
struct Hours(u32);
struct Days(u32);
```

These structs are called **Tuple Structs**, they are similar to regular structs but they have unnamed fields. You can access the fields of a tuple struct using the index of the field, e.g. `Hours(1).0` will return `1`.

You need to implement the `From` trait for the following conversions:

- `Minutes` to `Hours`
- `Hours` to `Days`
- `Minutes` to `Days`
- `Days` to `Hours`

## Example

```rust
let minutes = Minutes(60);
let hours: Hours = minutes.into();
assert_eq!(hours.0, 1);
```

Good luck!
