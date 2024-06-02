mod tests;

pub fn describe_number(n: i32) -> String {
    if n == 0 {
        return String::from("Zero");
    } else if n > 0 && n % 2 == 0 {
        return String::from("Positive even");
    } else if n > 0 && n % 2 != 0 {
        return String::from("Positive odd");
    } else if n < 0 && n % 2 == 0 {
        return String::from("Negative even");
    } else {
        return String::from("Negative odd");
    }
}
