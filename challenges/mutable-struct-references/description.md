Rust allows structs to hold mutable references to owned data, such as a `String`, enabling them to modify the data in-place. This requires a deep understanding of lifetimes and Rust's borrowing rules to ensure memory safety.

In this challenge, you will implement a struct named `MutableTextFinder` that holds a mutable reference to a `String`. This struct will allow for both searching and modifying the content of the `String`.

The `MutableTextFinder` struct should provide the following functionality:

- **`find_first`**: Searches for the first line containing a given keyword and returns it as an immutable reference (`Option<&str>`).
- **`replace_lines`**: Replaces all lines containing a given keyword with a replacement string.

## Your Task

1. Implement the `MutableTextFinder` struct that stores a mutable reference to a `String`.
2. Provide methods for:
   - Searching (`find_first`).
   - Replacing lines (`replace_lines`).

## Requirements

### Constraints

- The `MutableTextFinder` struct must include explicit lifetime annotations for the mutable reference.
- The `replace_lines` method should modify the `String` in-place.
- Lines are separated by the newline character `\n`.
- Searches should be case-sensitive.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- **Lifetime Annotations**: Ensure the lifetime annotations tie the struct's lifetime to the referenced `String`.
- **String Mutation**: Use `.lines()` to work with lines and `.replace()` or `.join()` for rebuilding the modified content.
- **Iterators**: Iterators can simplify the process of finding and replacing lines.

</details>
