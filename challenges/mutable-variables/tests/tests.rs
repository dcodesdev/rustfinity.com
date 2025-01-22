use mutable_variables::*;

#[test]
fn test_compiles() {
    mutating_variables();
}

#[test]
fn test_mutates_value_works() {
    let mut text = String::new();

    mutates_value(&mut text);

    assert_eq!(
        text, "bye",
        "The function `mutates_value` should not be edited"
    );
}

#[test]
fn test_return_value() {
    assert_eq!(
        mutating_variables(),
        "bye",
        "Expected the value to be 'bye'"
    );
}
