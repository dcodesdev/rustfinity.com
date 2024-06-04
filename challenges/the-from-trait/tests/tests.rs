#[cfg(test)]
mod tests {
    use the_from_trait::*;

    #[test]
    fn should_convert_minutes_to_hours() {
        let minutes = Minutes(120);
        let hours: Hours = minutes.into();
        assert_eq!(hours.0, 2);
    }

    #[test]
    fn should_convert_hours_to_days() {
        let hours = Hours(48);
        let days: Days = hours.into();
        assert_eq!(days.0, 2);
    }

    #[test]
    fn should_convert_minutes_to_days() {
        let minutes = Minutes(2880);
        let days: Days = minutes.into();
        assert_eq!(days.0, 2);
    }

    #[test]
    fn should_convert_days_to_hours() {
        let days = Days(2);
        let hours: Hours = days.into();
        assert_eq!(hours.0, 48);
    }
}
