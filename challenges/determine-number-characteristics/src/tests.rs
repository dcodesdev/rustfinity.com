#[cfg(test)]
mod tests {
    use crate::describe_number;

    #[test]
    fn test_positive_even() {
        assert_eq!(describe_number(10), "Positive even");
    }

    #[test]
    fn test_positive_odd() {
        assert_eq!(describe_number(7), "Positive odd");
    }

    #[test]
    fn test_negative_even() {
        assert_eq!(describe_number(-4), "Negative even");
    }

    #[test]
    fn test_negative_odd() {
        assert_eq!(describe_number(-9), "Negative odd");
    }

    #[test]
    fn test_zero() {
        assert_eq!(describe_number(0), "Zero");
    }
}
