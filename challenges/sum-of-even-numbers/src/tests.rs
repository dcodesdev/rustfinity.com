#[cfg(test)]
mod tests {
    use crate::sum_of_evens;

    #[test]
    fn test_sum_of_evens_range() {
        assert_eq!(sum_of_evens(1, 10), 30);
    }

    #[test]
    fn test_sum_of_evens_single_value() {
        assert_eq!(sum_of_evens(5, 5), 0);
    }

    #[test]
    fn test_sum_of_evens_reverse_range() {
        assert_eq!(sum_of_evens(10, 1), 0);
    }

    #[test]
    fn test_sum_of_evens_all_even() {
        assert_eq!(sum_of_evens(2, 6), 12); // 2 + 4 + 6 = 12
    }

    #[test]
    fn test_sum_of_evens_negative_range() {
        assert_eq!(sum_of_evens(-5, 5), 0); // -4 + -2 + 0 + 2 + 4 = 0
    }
}
