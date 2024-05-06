#[cfg(test)]
mod tests {
    use crate::parse_percentage;

    #[test]
    fn test_parse_percentage_valid() {
        assert_eq!(parse_percentage("75"), Ok(75));
    }

    #[test]
    fn test_parse_percentage_out_of_range() {
        assert_eq!(
            parse_percentage("150"),
            Err(String::from("Percentage out of range"))
        );
    }

    #[test]
    fn test_parse_percentage_non_numeric() {
        assert_eq!(parse_percentage("abc"), Err(String::from("Invalid input")));
    }

    #[test]
    fn test_parse_percentage_edge_cases() {
        assert_eq!(parse_percentage("0"), Ok(0));
        assert_eq!(parse_percentage("100"), Ok(100));
    }
}
