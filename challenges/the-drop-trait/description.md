The `Drop` trait in Rust is used to run custom cleanup logic when a value goes out of scope. This is particularly useful for managing resources like files, network connections, or any other system resources that need to be explicitly released.

When a value that implements `Drop` goes out of scope, the Rust runtime automatically calls the `drop` method, allowing you to define custom behavior for resource cleanup.

In this challenge, you will implement a struct that represents a temporary file. The file should be automatically deleted when the struct is dropped, simulating the behavior of temporary files in real-world applications.

## Your Task

1. Create `TempFile` struct with a method `new` that takes a file name as input, it must accept both `&str` and `String` types.
2. Implement the `Drop` trait for `TempFile` to delete the file automatically when the struct goes out of scope.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use `std::fs::File` to create a temporary file.
- Use `std::env::temp_dir()` to get the path for temporary files.
- The `std::fs::remove_file` method can be used to delete files.
- The `PathBuf` struct is helpful for managing file paths.
- Use the `AsRef<str>` trait to allow flexible input types for the file name.
- Implement the `Drop` trait for custom cleanup logic.

</details>
