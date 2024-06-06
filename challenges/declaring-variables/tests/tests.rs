use declaring_variables::*;
use syntest::{LocalValue, Syntest};

#[test]
fn test_calculate_area() {
    assert_eq!(calculate_area(), 50);
}

#[test]
fn test_variable_width() {
    let syntest = Syntest::from("./src/lib.rs");

    assert_eq!(
        syntest.get_local_value("calculate_area", "width").unwrap(),
        LocalValue::Int(10)
    );
    assert_eq!(
        syntest.get_local_value("calculate_area", "height").unwrap(),
        LocalValue::Int(5)
    );
}
