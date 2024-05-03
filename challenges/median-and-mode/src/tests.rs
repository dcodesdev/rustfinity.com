#[cfg(test)]
mod tests {
    use crate::{median, mode};

    #[test]
    fn test_median() {
        let mut numbers = vec![1, 2, 2, 3, 4];
        assert_eq!(median(&mut numbers), 2.0);

        let mut numbers = vec![6, 1, 2, 3, 4, 5];
        assert_eq!(median(&mut numbers), 3.5);

        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(median(&mut numbers), 4.0);

        let mut numbers = vec![123, 456, 789, 101112, 131415];
        assert_eq!(median(&mut numbers), 789.0);
    }

    #[test]
    fn test_mode() {
        let numbers = vec![1, 2, 2, 2, 3, 4];
        assert_eq!(mode(&numbers), vec![2]);

        let numbers = vec![1, 3, 3, 6, 7, 8, 9];
        assert_eq!(mode(&numbers), vec![3]);

        let numbers = vec![2, 2, 2, 2, 2, 2, 2, 100, 100, 100, 100, 100, 100, 100, 100];
        assert_eq!(mode(&numbers), vec![100]);

        let numbers = vec![1, 1, 2, 2];
        assert_eq!(mode(&numbers), vec![1, 2]);
    }
}
