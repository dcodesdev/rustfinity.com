pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    // TODO: Instead of using `unwrap`, use the `?` operator to propagate the option
    // HINT: `numbers.get` returns a Option<i32> value

    let num1 = numbers.get(index1).unwrap();
    let num2 = numbers.get(index2).unwrap();
    Some(num1 * num2)
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
