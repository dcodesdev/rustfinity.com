use declaring_variables::*;
use syntest::{Syntest, Value};

#[test]
fn test_calculate_area() {
    assert_eq!(calculate_area(), 50);
}

#[test]
fn test_variables() {
    let syntest = Syntest::from("./src/lib.rs");

    // Expect the 2 variables to exist
    let variables_to_exist = ["height", "width"];
    variables_to_exist.iter().for_each(|&v| {
        let var = syntest
            .var_details("calculate_area", v)
            .expect(&format!("Variable {} was not declared", v));

        assert!(var.is_used(), "Variable {v} was not used");
        assert_eq!(var.name(), v);

        if v == "width" {
            assert_eq!(var.value(), Value::Int(10), "Width should be 10");
        } else if v == "height" {
            assert_eq!(var.value(), Value::Int(5), "Height should be 5");
        }
    });
}
