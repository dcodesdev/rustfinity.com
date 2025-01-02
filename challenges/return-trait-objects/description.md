In Rust, polymorphism is achieved using traits. When working with objects of different types that share a common behavior (defined by a trait), you can use trait objects. Trait objects allow dynamic dispatch, enabling flexibility and extensibility.

The most common way to use a trait object is through `Box<dyn Trait>`, which encapsulates an object in the heap and exposes the behavior defined by the trait. This challenge will teach you to define and return such trait objects in Rust.

## Your Task

You are tasked with implementing a trait called `Speakable`, which defines a behavior for objects that can "speak." Create two structs, `Dog` and `Robot`, each having two named fields. Both structs should implement the `Speakable` trait. Finally, write a function `get_speaker` that takes a `&str` parameter (`"dog"` or `"robot"`) and returns a `Box<dyn Speakable>`.

### Requirements

#### Define the `Speakable` Trait

- The trait should have a method `speak` that returns a `String`.

#### Define the `Dog` and `Robot` Structs

- `Dog` should have:
  - A `name: String` field for the dog's name.
  - A `breed: String` field for the dog's breed.
- `Robot` should have:
  - A `model: String` field for the robot's model name.
  - A `purpose: String` field describing the robot's purpose.

#### Implement the `Speakable` Trait

- For `Dog`, `speak` should return a string like `"Woof! I am [name], the [breed] dog."`.
- For `Robot`, `speak` should return a string like `"Beep boop. I am [model], built for [purpose]."`.

#### Write the `get_speaker` Function

- The function should:
  - Take a `&str` parameter (`"dog"` or `"robot"`).
  - Create a new `Dog` or `Robot` instance with hardcoded field values.
  - Return a `Box<dyn Speakable>` containing the appropriate instance.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- Use `impl TraitName for StructName` to implement a trait for a struct.
- Use `Box::new(instance)` to encapsulate an object for returning as `Box<dyn Trait>`.
- Use string interpolation (`format!`) to construct the `speak` messages dynamically.

</details>
