#[cfg(test)]
mod tests {
    use ownership::calculate_length;

    #[test]
    fn test_calculate_length() {
        assert_eq!(
            calculate_length(&String::from("hello")),
            5,
            "Expected the length of 'hello' to be 5"
        );

        assert_eq!(
            calculate_length(&String::from("world")),
            5,
            "Expected the length of 'world' to be 5"
        );

        assert_eq!(
            calculate_length(&String::from("hello world")),
            11,
            "Expected the length of 'hello world' to be 11"
        );

        assert_eq!(
            calculate_length(&String::from("hello world!")),
            12,
            "Expected the length of 'hello world!' to be 12"
        );

        assert_eq!(
            calculate_length(&String::from("I'm a rust developer, and I love it!")),
            36,
            "Expected the text to be 39 characters long"
        );
    }
}
