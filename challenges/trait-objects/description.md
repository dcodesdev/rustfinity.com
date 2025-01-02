In Rust, trait objects are a powerful way to achieve polymorphism. They allow you to work with different types that implement the same trait. This becomes particularly useful when you want to store objects of varying types in the same struct while ensuring they conform to a shared interface.

This challenge focuses on using `Box<dyn Trait>` as fields in structs to encapsulate objects with dynamic behavior.

## Your Task

You need to define a trait called `Renderable` and create two structs, `Circle` and `Rectangle`, that implement this trait. Then, you will create another struct called `Canvas`, which can hold a collection of objects implementing the `Renderable` trait. The `Canvas` struct should have methods to add objects and render all objects.

### Requirements

#### Define the `Renderable` Trait

- The trait should have a method `render(&self) -> String` to represent the object visually.

#### Define the `Circle` and `Rectangle` Structs

- `Circle` should have a `radius: f64` field.
- `Rectangle` should have `width: f64` and `height: f64` fields.

#### Implement the `Renderable` Trait

- For `Circle`, the `render` method should return a string like `"Circle with radius X"`.
- For `Rectangle`, the `render` method should return a string like `"Rectangle with width X and height Y"`.

#### Define the `Canvas` Struct

- The struct should have a `shapes: Vec<Box<dyn Renderable>>` field.
- Implement the following methods for `Canvas`:
  - `new() -> Canvas`: Initializes an empty canvas.
  - `add_shape(&mut self, shape: Box<dyn Renderable>)`: Adds a shape to the canvas.
  - `render_all(&self) -> Vec<String>`: Returns a vector of strings, each representing a rendered shape.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- Use `Box<dyn Trait>` to store trait objects in the `shapes` vector.
- The `render` method should use string interpolation (`format!`) to generate its output.
- Remember to derive or implement the `Default` trait for `Canvas` if needed for `new()`.

</details>
