// Finish the function
pub fn filter_even_numbers

// Example usage
pub fn main() {
    let input = [1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<&i32> = filter_even_numbers(&input).collect();
    println!("{:?}", even_numbers); // Expected output: [&2, &4, &6]
}
