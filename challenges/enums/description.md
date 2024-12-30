Enums are a core part of Rust, allowing you to define a type that can be one of several distinct variants. Enums can hold data and make it easier to work with related states or choices.

In this challenge, you will implement a simple `TrafficLight` enum to represent the states of a traffic light: `Red`, `Yellow`, and `Green`. Each variant will also include the duration (in seconds) the light should stay on.

## Your Task

Create an enum `TrafficLight` with three variants:

- `Red(u8)` representing the red light with its duration in seconds.
- `Yellow(u8)` representing the yellow light with its duration in seconds.
- `Green(u8)` representing the green light with its duration in seconds.

Write a function `light_duration` that takes a `TrafficLight` and returns its duration.

### Example

```rust
let red = TrafficLight::Red(30);
assert_eq!(light_duration(red), 30);

let green = TrafficLight::Green(60);
assert_eq!(light_duration(green), 60);
```

### Requirements

- Define the `TrafficLight` enum with three variants.
- Write the function `light_duration` to return the duration for each variant.
- Handle each variant using a `match` expression.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `match` statement to handle each enum variant and extract the associated value.
- Remember that enum variants can hold data like a tuple.
- The `match` expression must cover all possible variants of the enum.

</details>
