pub fn filter_even_numbers(input: &[i32]) -> impl Iterator<Item = &i32> {
    input.into_iter().filter(|&&x| x % 2 == 0)
}
