Working with strings is a common requirement in real-world applications. Sometimes, you want a function that can take either a `String` or a `&str` as input without needing separate implementations. Rust's `Into` trait allows you to do just that, enabling type conversion in a seamless way.

The `Into` trait is a powerful standard library trait that allows a type to be converted into another type. It’s especially useful in function arguments to handle multiple types generically. For example, it can be used to create a function that works with both owned (`String`) and borrowed (`&str`) string types.

## Your Task

Implement a function `print_message` that accepts any type that can be converted into a `String`. Use the `Into<String>` trait bound to allow the function to handle both `String` and `&str` inputs. The function should:

- Accept a single argument of a generic type that implements `Into<String>`.
- Convert the input into a `String` and print it to the console.

## Requirements

### Constraints

- Use the `Into` trait to handle type conversion.
- The function must work for both `String` and `&str` without requiring explicit conversions in the function call.
- The function should print the message to the console using `println!`.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The `Into` trait is implemented by many types in Rust, including `&str` and `String` for `String`.
- Use the `.into()` method to perform the type conversion.
- Consider using a generic type parameter with the `Into<String>` bound in your function signature.

</details>
