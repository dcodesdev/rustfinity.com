use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Input is not a valid number"),
            ParsePercentageError::OutOfRange => write!(f, "Number is out of range (0-100)"),
        }
    }
}

impl Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(num) if num <= 100 => Ok(num),
        Ok(_) => Err(ParsePercentageError::OutOfRange),
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}
