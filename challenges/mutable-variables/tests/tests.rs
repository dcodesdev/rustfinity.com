#[cfg(test)]
mod tests {
    use mutable_variables::*;
    use syntest::{LocalVariable, Mutation, Syntest, Value};

    #[test]
    fn test_compiles() {
        manipulate_variables();
    }

    #[test]
    fn test_variables() {
        let syntest = Syntest::from("./src/lib.rs");

        // Expect the 2 variables to exist
        syntest
            .variables("manipulate_variables")
            .iter()
            .for_each(|var| match var {
                LocalVariable::Int { .. } => match var.name() {
                    "x" => {
                        assert_eq!(
                            var.value(),
                            Value::Int(15),
                            "Expected x to be 15, got {}",
                            var.value()
                        );
                        assert!(var.is_used(), "Expected x to be used");
                        assert!(var.is_mutable(), "Expected x to be mutable");
                        assert_eq!(
                            var.mutations(),
                            &vec![Mutation::new(Value::Int(5), Value::Int(15))],
                            "Expected variable x to have a mutation from 5 to 15.",
                        );
                    }
                    "y" => {
                        assert_eq!(
                            var.value(),
                            Value::Int(20),
                            "Expected y to be 20, got {}",
                            var.value()
                        );
                        assert!(var.is_used(), "Expected y to be used");
                        assert!(var.is_mutable(), "Expected y to be mutable");
                        assert_eq!(
                            var.mutations(),
                            &vec![Mutation::new(Value::Int(10), Value::Int(20))],
                            "Expected variable y to have a mutation from 10 to 20."
                        );
                    }
                    _ => {
                        panic!("Unexpected variable {}", var.name())
                    }
                },
                _ => {
                    panic!("Expected type to be an integer.")
                }
            })
    }
}
