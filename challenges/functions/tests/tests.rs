#[cfg(test)]
mod tests {
    use functions::{add, multiply, subtract};

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(134, 122), 256);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(3, 5), -2);
        assert_eq!(subtract(0, 0), 0);
        assert_eq!(subtract(134, 122), 12);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-1, 1), -1);
        assert_eq!(multiply(0, 3), 0);
        assert_eq!(multiply(134, 122), 16348);
    }
}
