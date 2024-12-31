pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&num| num % 2 == 0).copied()
}
