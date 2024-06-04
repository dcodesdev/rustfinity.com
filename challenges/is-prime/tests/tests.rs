#[cfg(test)]
mod tests {
    use is_prime::*;

    #[test]
    fn test_is_prime_small_numbers() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_prime_large_numbers() {
        assert_eq!(is_prime(29), true);
        assert_eq!(is_prime(49), false);
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_is_prime_even_numbers() {
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(14), false);
    }

    #[test]
    fn test_is_prime_odd_numbers() {
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
    }
}
