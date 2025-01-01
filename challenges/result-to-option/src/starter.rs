pub fn read_first_line_as_option(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()`. Use `?` to propagate errors.
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_first_line_as_option(file_path) {
        Some(line) => println!("First line: {}", line),
        None => println!("Failed to read the first line."),
    }
}
