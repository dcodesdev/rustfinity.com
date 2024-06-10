#[cfg(test)]
mod tests {
    use declaring_variables::*;
    use syntest::{Syntest, Value};

    #[test]
    fn test_calculate_area() {
        assert_eq!(calculate_area(), 50);
    }

    #[test]
    fn test_variables() {
        let syntest = Syntest::new("calculate_area", "src/lib.rs");

        // Expect the 2 variables to exist
        let variables_to_exist = ["height", "width"];
        variables_to_exist.iter().for_each(|&v| {
            let var = syntest
                .var_details(v)
                .expect(&format!("Variable {} was not declared", v));

            assert!(var.is_used(), "Variable {v} was not used");
            assert_eq!(var.name(), v);

            if v == "width" {
                assert_eq!(var.value(), Value::Int(10), "Width should be 10");
            } else if v == "height" {
                assert_eq!(var.value(), Value::Int(5), "Height should be 5");
            }

            assert_eq!(
                syntest.count_var(v),
                1,
                "Variable {v} should be defined once"
            );

            assert_eq!(var.is_mutable(), false, "Variable {v} should be immutable");
        });
    }
}
