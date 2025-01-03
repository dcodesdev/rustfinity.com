In Rust, structs can store references in addition to owned values. This allows you to create powerful and memory-efficient abstractions by reusing data rather than duplicating it. However, using references in structs requires careful attention to lifetimes to ensure that the references remain valid.

In this challenge, you will create a struct named `TextFinder` that holds a reference to a string slice (`&str`). This struct will be used to perform search operations on the string slice by finding lines that contain a given keyword. The `TextFinder` struct should provide two methods:

- `find_first`: Returns the first line containing the keyword, or `None` if no match is found.
- `find_many`: Returns a vector of all lines containing the keyword.

## Your Task

Implement the `TextFinder` struct with the following functionality:

1. It must store a reference to a string slice (`&str`).
2. It should include the `find_first` and `find_many` methods:
   - `find_first` takes a keyword as input and returns an `Option<&str>` for the first matching line.
   - `find_many` takes a keyword as input and returns a `Vec<&str>` of all matching lines.

## Requirements

### Constraints

- The `TextFinder` struct must include explicit lifetime annotations to ensure that the reference it holds is valid throughout its usage.
- The search functionality should be case-sensitive.
- Lines are separated by the newline character `\n`.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- **Lifetime Annotations**: Use a lifetime annotation to link the struct's lifetime to the lifetime of the string slice it references. For example: `struct TextFinder<'a> { ... }`.
- **String Methods**: You can use `.lines()` on a `&str` to split it into lines and `.contains()` to check if a string contains a substring.
- **Iterators**: Iterators like `.find()` or `.filter()` can simplify search operations.
- **Returning References**: Ensure you return references to the original string slice rather than creating new owned strings.

</details>
