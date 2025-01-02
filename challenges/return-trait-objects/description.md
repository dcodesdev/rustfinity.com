Sometimes, the Rust compiler cannot determine the size of a trait object at compile time. For example, if a function returns a trait object based on a condition, in such cases, `impl Trait` is not going to work. This is because the size of the trait object is not known at compile time. In such cases, you can use `Box<dyn Trait>` to return a trait object.

`Box<T>` lets you allocate memory on the heap and store the value of `T` in that memory. `Box<dyn Trait>` is a trait object that allows you to store a value of any type that implements `Trait` in the heap.

This is called **dynamic dispatch** because the method to call is determined at runtime, not at compile time. This is in contrast to **static dispatch**, where the method to call is determined at compile time.

## Your Task

You need to define a function that returns a `Box<dyn Speakable>` based on a condition.

Here is what you need to do:

- Define the `Speakable` trait with a method `speak` that returns a `String`.
- Define a struct `Dog` with two fields: `name` and `breed`, both of type `String`.
- Implement the `Speakable` trait for `Dog` to return a string `Woof`.
- Define a struct `Robot` with two fields: `model` and `purpose`, both of type `String`.
- Implement the `Speakable` trait for `Robot` to return a string `Beep boop`.
- Finish the function `get_speaker` that takes a `&str` parameter and returns either a `Dog` or a `Robot` based on the parameter.
- The parameter can either be `dog` or `robot`.
