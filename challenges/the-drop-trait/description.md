The `Drop` trait in Rust is used to run custom cleanup logic when a value goes out of scope. This is particularly useful for managing resources like files, network connections, or any other system resources that need to be explicitly released.

When a value that implements `Drop` goes out of scope, the Rust runtime automatically calls the `drop` method, allowing you to define custom behavior for resource cleanup.

In this challenge, you will implement a struct that represents a temporary file. The file should be automatically deleted when the struct is dropped, simulating the behavior of temporary files in real-world applications.

## Your Task

Your task is to implement the `Drop` trait for a struct `TempFile`. This struct should:

1. Create a temporary file with a user-specified name when initialized.
2. Delete the file when it goes out of scope.

The implementation should ensure that the file cleanup happens even if the program ends abruptly or panics.

## Requirements

### Functional Requirements

- Implement a struct `TempFile` with:
  - A method `new` that takes a file name as input (using `AsRef<str>` for flexibility).
  - A field to store the file's path.
- Implement the `Drop` trait for `TempFile` to delete the file automatically.
- Use `std::fs` for file operations.

### Constraints

- The method `new` should accept both `String` and `&str` types for the file name.
- Handle potential errors during file creation or deletion gracefully.
- Ensure the file cleanup occurs under all circumstances (e.g., panics).
- Do not use external crates; stick to the Rust standard library.

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
