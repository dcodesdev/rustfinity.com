mod tests;

pub fn parse_percentage(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(percentage) if percentage <= 100 => Ok(percentage),
        Ok(_) => Err(String::from("Percentage out of range")),
        Err(_) => Err(String::from("Invalid input")),
    }
}
