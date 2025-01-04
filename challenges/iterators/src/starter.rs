pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    // Your code here...
}

pub fn uppercase_strings(iter: impl Iterator<Item = &str>) -> Vec<String> {
    // Your code here...
}

// Example usage
pub fn main() {
    // Filtering even numbers
    let numbers = vec![1, 2, 3, 4, 5];
    let odd_numbers = filter_even_numbers(numbers.into_iter());
    println!("{:?}", odd_numbers); // Should print: [1, 3, 5]

    // Converting strings to uppercase
    let words = vec!["hello", "world"];
    let uppercase_words = uppercase_strings(words.into_iter());
    println!("{:?}", uppercase_words); // Should print: ["HELLO", "WORLD"]
}
