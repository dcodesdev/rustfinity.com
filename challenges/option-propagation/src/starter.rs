pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    // TODO: Implement this function
    // Hint: Use the `get` method and the `?` operator to propagate `None`.
}

// Example usage
pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Example 1: Both indices are valid
    let result = find_and_multiply(numbers.clone(), 1, 3);
    println!("{:?}", result); // Should print Some(8)

    // Example 2: One index is out of bounds
    let result = find_and_multiply(numbers.clone(), 1, 10);
    println!("{:?}", result); // Should print None
}
