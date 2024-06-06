use declaring_variables::*;
use syntest::Syntest;

#[test]
fn test_calculate_area() {
    assert_eq!(calculate_area(), 50);
}

#[test]
fn test_variable_width() {
    let syntest = Syntest::from("./src/lib.rs");
}
