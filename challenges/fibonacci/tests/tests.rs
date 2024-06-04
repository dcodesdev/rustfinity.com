#[cfg(test)]
mod tests {
    use fibonacci::*;

    #[test]
    fn zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn more_than_one() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
    }
}
