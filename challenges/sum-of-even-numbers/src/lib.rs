pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    if start > end {
        return 0;
    }

    let mut sum = 0;
    for number in start..=end {
        if number % 2 == 0 {
            sum += number;
        }
    }
    sum
}
