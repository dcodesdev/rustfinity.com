#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_factorial_of_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_of_small_numbers() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn test_factorial_of_large_numbers() {
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(20), 2432902008176640000);
    }
}
