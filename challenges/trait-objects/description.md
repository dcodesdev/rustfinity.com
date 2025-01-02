Now that we know a little bit about trait objects and how to work with them, let's use them in a different example, this challenge focuses on using `Box<dyn Trait>` as fields in structs to encapsulate objects with dynamic behavior.

## Your Task

You need to define a trait called `Renderable` and create two structs, `Circle` and `Rectangle`, that implement this trait. Then, you will create another struct called `Canvas`, which can hold a collection of objects implementing the `Renderable` trait. The `Canvas` struct should have methods to add objects and render all objects.

### Requirements

Here are the requirements for the program, make sure to read them carefully:

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
  - `add_shape()`: Adds a shape to the canvas.
  - `render_all()`: Returns a vector of strings, each representing a rendered shape.
