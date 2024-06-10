#[cfg(test)]
mod tests {
    use mutable_variables::*;
    use syntest::{LocalVariable, Syntest};

    #[test]
    fn test_compiles() {
        mutating_variables();
    }

    #[test]
    fn test_variables() {
        let syntest = Syntest::new("mutating_variables", "./src/lib.rs");

        let mut success = false;
        // Expect the 2 variables to exist
        syntest.variables().iter().for_each(|var| match var {
            LocalVariable::Str { .. } => match var.name() {
                "text" => {
                    assert!(var.is_used(), "Expected 'text' to be used");
                    assert!(var.is_mutable(), "Expected 'text' to be mutable");
                    assert!(
                        var.mutations().len() >= 1,
                        "Expected 'text' to be mutated at least once"
                    );
                    success = true;
                }
                _ => {}
            },
            _ => {}
        });

        assert!(success, "Expected 'text' to be defined");
    }
}
