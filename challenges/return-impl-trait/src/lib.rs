pub fn filter_starts_with<'a>(
    input: &'a [String],
    keyword: &'a str,
) -> impl Iterator<Item = &'a String> {
    input.iter().filter(move |s| s.starts_with(keyword))
}
