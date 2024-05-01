mod tests;

pub fn fizz_buzz(num: u32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => num.to_string(),
    }
}
