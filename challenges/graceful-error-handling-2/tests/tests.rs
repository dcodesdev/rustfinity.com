use graceful_error_handling_2::*;

fn implements_debug<T: std::fmt::Debug>(_: T) {}
fn implements_display<T: std::fmt::Display>(_: T) {}
fn implements_error<T: std::error::Error>(_: T) {}

#[test]
fn test_parse_percentage_valid() {
    assert_eq!(parse_percentage("75"), Ok(75));
}

#[test]
fn test_parse_percentage_out_of_range() {
    assert_eq!(
        parse_percentage("150"),
        Err(ParsePercentageError::OutOfRange)
    );
}

#[test]
fn test_parse_percentage_invalid_input() {
    assert_eq!(
        parse_percentage("abc"),
        Err(ParsePercentageError::InvalidInput)
    );
}

#[test]
fn test_parse_percentage_edge_cases() {
    assert_eq!(parse_percentage("0"), Ok(0));
    assert_eq!(parse_percentage("100"), Ok(100));
}

#[test]
fn test_implements_debug() {
    implements_debug(ParsePercentageError::InvalidInput);
    implements_debug(ParsePercentageError::OutOfRange);
}

#[test]
fn test_implements_display() {
    implements_display(ParsePercentageError::InvalidInput);
    implements_display(ParsePercentageError::OutOfRange);
}

#[test]
fn test_implements_error() {
    implements_error(ParsePercentageError::InvalidInput);
    implements_error(ParsePercentageError::OutOfRange);
}
