Rust allows structs to hold mutable references to owned data, enabling them to modify the data in-place. This requires a deep understanding of lifetimes and Rust's borrowing rules to ensure memory safety.

## Your Task

In this challenge, you will implement a struct named `MutableTextFinder` that holds a mutable reference to a `String`. This struct will allow for both searching and modifying the content of the `String`.

The `MutableTextFinder` struct should provide the following functionality:

- `find_first`: Searches for the first line containing a given keyword and returns it as an immutable reference (`Option<&str>`).
- `replace_lines`: Replaces all lines containing a given keyword with a replacement string.
- `get_text`: Returns the reference to the content.
- Searches should be case-sensitive.
