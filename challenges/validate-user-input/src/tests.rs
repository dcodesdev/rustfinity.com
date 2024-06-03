#[cfg(test)]
mod tests {
    use crate::validate_user;

    #[test]
    fn test_valid_user() {
        assert_eq!(validate_user(25, "user@example.com"), Ok(()));
    }

    #[test]
    fn test_invalid_age() {
        assert_eq!(
            validate_user(-1, "user@example.com"),
            Err(String::from("Invalid age"))
        );
        assert_eq!(
            validate_user(121, "user@example.com"),
            Err(String::from("Invalid age"))
        );
    }

    #[test]
    fn test_invalid_email() {
        assert_eq!(
            validate_user(25, "userexample.com"),
            Err(String::from("Invalid email"))
        );
    }

    #[test]
    fn test_invalid_age_and_email() {
        assert_eq!(
            validate_user(-1, "userexample.com"),
            Err(String::from("Invalid age"))
        );
    }
}
