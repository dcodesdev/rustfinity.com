#[cfg(test)]
mod tests {
    use find_the_first_palindrome::*;

    #[test]
    fn test_find_first_palindrome_in_range() {
        assert_eq!(find_first_palindrome(10, 30), Some(11));
    }

    #[test]
    fn test_find_first_palindrome_no_palindromes() {
        assert_eq!(find_first_palindrome(123, 130), None);
    }

    #[test]
    fn test_find_first_palindrome_single_palindrome() {
        assert_eq!(find_first_palindrome(100, 105), Some(101));
    }

    #[test]
    fn test_find_first_palindrome_reversed_range() {
        assert_eq!(find_first_palindrome(30, 10), Some(11));
    }

    #[test]
    fn test_find_first_palindrome_edge_case() {
        assert_eq!(find_first_palindrome(1, 1), Some(1));
    }

    #[test]
    fn test_find_first_palindrome_negative_range() {
        assert_eq!(find_first_palindrome(-1, 1), Some(0));
    }
}
