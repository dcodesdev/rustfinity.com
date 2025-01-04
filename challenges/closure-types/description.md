Closures in Rust are categorized based on how they interact with the environment and the variables they capture. The three main types are `Fn`, `FnMut`, and `FnOnce`. Understanding and using these types effectively is key to mastering closures in Rust.

### Closure Type Definitions

- **Fn**: A closure that borrows its environment immutably. It is useful for read-only operations.
- **FnMut**: A closure that mutably borrows its environment, allowing it to modify variables.
- **FnOnce**: A closure that takes ownership of its captured variables, consuming them in the process.

### Practical Example

Imagine we are managing a **shopping cart** system:

1. A **calculator** computes the total price of an item, including tax (immutable operation, `Fn`).
2. A **discount applier** adjusts the cart's total price by applying a discount (mutable operation, `FnMut`).
3. A **checkout processor** consumes the cart's contents and returns the final receipt (ownership operation, `FnOnce`).

## Your Task

Implement the following closures and their respective behaviors:

- `calculate_total`: An `Fn` closure that calculates the total price of an item, including tax (`price + price * tax_rate`).
- `apply_discount`: An `FnMut` closure that mutates the cart total by subtracting a given discount.
- `checkout_cart`: An `FnOnce` closure that consumes the cart's details (a `String`) and returns a confirmation message.

## Requirements

1. Define these closures explicitly as `Fn`, `FnMut`, and `FnOnce`.
2. Implement them in the function `create_typed_closures` so they can be returned as a tuple.
3. Use them to simulate the shopping cart example.

### Example

```rust
let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

assert_eq!(calculate_total(100.0, 0.2), 120.0);

let mut total_price = 120.0;
apply_discount(&mut total_price, 20.0);
assert_eq!(total_price, 100.0);

let cart_details = String::from("Items: Apple, Banana, Orange");
assert_eq!(
    checkout_cart(cart_details),
    "Checkout complete: Items: Apple, Banana, Orange"
);
```

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use `impl Fn`, `impl FnMut`, or `impl FnOnce` to define the types of the closures.
- For `FnMut`, use mutable references to update the cart total.
- For `FnOnce`, the closure will take ownership of its input, so it cannot be reused.

</details>
