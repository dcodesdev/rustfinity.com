// 1. Finish the definition
pub enum ParsePercentageError

// 2. Implement the `Error` trait

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
