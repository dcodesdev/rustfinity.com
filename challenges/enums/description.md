Enums are a fundamental feature in Rust that let you define a type that can be one of several distinct variants. Enums make it easier to represent a set of related states or options cleanly and effectively.

In this challenge, you will implement a simple `TrafficLight` enum to represent the states of a traffic light: `Red`, `Yellow`, and `Green`. Each variant will be a unit struct with no associated data.

## Your Task

Create an enum `TrafficLight` with three variants:

- `Red` representing the red light.
- `Yellow` representing the yellow light.
- `Green` representing the green light.

Write a function `light_action` that takes a `TrafficLight` and returns a string describing the action associated with the light.

- For `Red`, return `"Stop"`.
- For `Yellow`, return `"Caution"`.
- For `Green`, return `"Go"`.

### Requirements

- Define the `TrafficLight` enum with three unit variants.
- Write the function `light_action` to return the correct action for each light.
- Use a `match` expression to handle the variants.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `match` statement to handle each enum variant.
- Each match arm should return a string corresponding to the light’s action.
- Unit variants do not hold any data, so you only need to match on the variant name.

</details>
