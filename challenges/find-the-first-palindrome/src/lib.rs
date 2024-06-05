pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };
    for number in start..=end {
        let s = number.to_string();
        if s == s.chars().rev().collect::<String>() {
            return Some(number);
        }
    }
    None
}
