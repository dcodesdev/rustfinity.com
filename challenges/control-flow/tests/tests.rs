#[cfg(test)]
mod tests {
    use control_flow::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(check_number_sign(10), "positive");
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(check_number_sign(-5), "negative");
    }

    #[test]
    fn test_zero() {
        assert_eq!(check_number_sign(0), "zero");
    }

    #[test]
    fn test_large_positive_number() {
        assert_eq!(check_number_sign(1000000), "positive");
    }

    #[test]
    fn test_large_negative_number() {
        assert_eq!(check_number_sign(-1000000), "negative");
    }
}
