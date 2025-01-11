#[macro_export]
macro_rules! math_operations {
    // TODO: Implement the macro
}

fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6",);
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7",);
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24",);
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5",);
}
