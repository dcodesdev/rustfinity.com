#[cfg(test)]
mod tests {
    use if_else::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }
}
