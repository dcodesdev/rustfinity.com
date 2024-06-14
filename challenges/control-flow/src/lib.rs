pub fn check_number_sign(number: i32) -> String {
    if number > 0 {
        "positive".to_string()
    } else if number < 0 {
        "negative".to_string()
    } else {
        "zero".to_string()
    }
}
