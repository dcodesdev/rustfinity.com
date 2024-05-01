#[cfg(test)]
mod tests {
    use crate::math_operations;

    #[test]
    fn math_tests() {
        let (sum, difference, multiply, divide) = math_operations(10, 5);

        assert_eq!(sum, 15);
        assert_eq!(difference, 5);
        assert_eq!(multiply, 50);
        assert_eq!(divide, 2);
    }

    #[test]
    fn with_large_numbers() {
        let (sum, difference, multiply, divide) = math_operations(1000, 5000);

        assert_eq!(sum, 6000);
        assert_eq!(difference, -4000);
        assert_eq!(multiply, 5000000);
        assert_eq!(divide, 0);
    }

    #[test]
    fn with_negative_numbers() {
        let (sum, difference, multiply, divide) = math_operations(-10, 5);

        assert_eq!(sum, -5);
        assert_eq!(difference, -15);
        assert_eq!(multiply, -50);
        assert_eq!(divide, -2);
    }

    #[test]
    fn with_mixed_numbers() {
        let (sum, difference, multiply, divide) = math_operations(-10, 5);

        assert_eq!(sum, -5);
        assert_eq!(difference, -15);
        assert_eq!(multiply, -50);
        assert_eq!(divide, -2);
    }
}
