#[cfg(test)]
mod tests {
    use sum_of_array::*;

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(&arr), 15);
    }

    #[test]
    fn test_sum_array_with_negatives() {
        let arr = [-1, -2, -3, -4, -5];
        assert_eq!(sum_array(&arr), -15);
    }

    #[test]
    fn test_sum_array_with_mixed() {
        let arr = [1, -2, 3, -4, 5];
        assert_eq!(sum_array(&arr), 3);
    }

    #[test]
    fn test_sum_array_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(sum_array(&arr), 0);
    }

    #[test]
    fn test_large_array() {
        let arr = [2; 1000];
        assert_eq!(sum_array(&arr), 2000);
    }
}
