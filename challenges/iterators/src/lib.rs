pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    iter.filter(|&x| x % 2 != 0).collect()
}

pub fn uppercase_strings<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<String> {
    iter.map(|s| s.to_uppercase()).collect()
}
