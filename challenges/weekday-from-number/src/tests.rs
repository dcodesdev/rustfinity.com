#[cfg(test)]
mod tests {
    use crate::weekday_from_number;

    #[test]
    fn test_weekday_from_number_valid() {
        assert_eq!(weekday_from_number(1), "Monday");
        assert_eq!(weekday_from_number(2), "Tuesday");
        assert_eq!(weekday_from_number(3), "Wednesday");
        assert_eq!(weekday_from_number(4), "Thursday");
        assert_eq!(weekday_from_number(5), "Friday");
        assert_eq!(weekday_from_number(6), "Saturday");
        assert_eq!(weekday_from_number(7), "Sunday");
    }

    #[test]
    fn test_weekday_from_number_invalid() {
        assert_eq!(weekday_from_number(0), "Invalid day number");
        assert_eq!(weekday_from_number(8), "Invalid day number");
    }
}
