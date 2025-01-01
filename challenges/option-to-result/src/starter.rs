pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // TODO: Implement this function
    // Hint: Use `Vec::first`, `.ok_or()`, and `if` statements for validation.
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
