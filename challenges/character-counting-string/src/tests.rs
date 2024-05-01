#[cfg(test)]
mod tests {
    use crate::count_characters;

    #[test]
    fn should_count_characters() {
        assert_eq!(count_characters("hello"), 5);
        assert_eq!(count_characters("hello world"), 11);
        assert_eq!(count_characters("hello world!"), 12);
        assert_eq!(count_characters("hello world!  "), 14);
        assert_eq!(count_characters("hello world!  123"), 17);
        assert_eq!(count_characters("hello world!  123  "), 19);
    }

    #[test]
    fn empty_string() {
        assert_eq!(count_characters(""), 0);
    }

    #[test]
    fn single_character() {
        assert_eq!(count_characters("a"), 1);
    }

    #[test]
    fn single_word() {
        assert_eq!(count_characters("hello"), 5);
    }
}
