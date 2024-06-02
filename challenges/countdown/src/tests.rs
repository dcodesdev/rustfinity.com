#[cfg(test)]
mod tests {
    use crate::countdown;

    #[test]
    fn test_countdown() {
        let result = countdown(3);
        assert_eq!(result, vec![3, 2, 1, 0]);
    }

    #[test]
    fn test_countdown_zero() {
        let result = countdown(0);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_countdown_large() {
        let result = countdown(10000);
        assert_eq!(result, (0..=10000).rev().collect::<Vec<u32>>());
    }
}
