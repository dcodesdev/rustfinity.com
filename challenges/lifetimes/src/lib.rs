pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}
