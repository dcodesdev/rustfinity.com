#[cfg(test)]
mod tests {
    use mutable_variables::*;
    use syntest::{LocalVariable, Mutation, Syntest, Value};

    #[test]
    fn test_compiles() {
        mutating_variables();
    }

    #[test]
    fn test_variables() {
        let syntest = Syntest::from("./src/lib.rs");

        // Expect the 2 variables to exist
        syntest
            .variables("mutating_variables")
            .iter()
            .for_each(|var| match var {
                LocalVariable::Str { value, .. } => match var.name() {
                    "text" => {
                        assert_eq!(
                            value.as_str(),
                            "bye",
                            "Expected text to be 'bye', got {}",
                            value.as_str()
                        );
                        assert!(var.is_used(), "Expected 'text' to be used");
                        assert!(var.is_mutable(), "Expected 'text' to be mutable");
                        assert_eq!(
                            var.mutations(),
                            &vec![Mutation::new(
                                Value::Str("hello".to_string()),
                                Value::Str("bye".to_string())
                            )],
                            "Expected variable 'text' to have mutated from 'hello' to 'bye'",
                        );
                    }
                    _ => {
                        panic!("Unexpected variable {}", var.name())
                    }
                },
                _ => {
                    panic!("Unexpected type for variable {}.", var.name())
                }
            });

        let macros = syntest.mac.macros("mutating_variables");

        assert_eq!(
            macros.len(),
            2,
            "Expected 2 println! macros to be used, got {}",
            macros.len()
        );

        match &macros[..] {
            [first, second] => {
                // First macro should be println!("Text is: {}", "hello");
                assert_eq!(first.tokens.len(), 3);
                // will have 3 tokens: &str, , and "hello"
                match &first.tokens[..] {
                    [text_is, _comma, text] => {
                        assert_eq!(text_is, "\"Text is: {}\"");
                        assert_eq!(
                            text, "text",
                            "Expected text variable to be used in println!"
                        );
                    }
                    _ => {
                        panic!(
                            "The println! macro should take exactly 2 arguments, got {} arguments",
                            first.tokens.len()
                        );
                    }
                }

                // Second macro should be println!("Text is: {}", "bye");
                assert_eq!(second.tokens.len(), 3);
                // will have 3 tokens: &str, , and "bye"
                match &second.tokens[..] {
                    [text_is, _comma, bye] => {
                        assert_eq!(text_is, "\"Text is: {}\"");
                        assert_eq!(bye, "text", "Expected text variable to be used in println!");
                    }
                    _ => {
                        panic!(
                            "The println! macro should take exactly 2 arguments, got {} arguments",
                            second.tokens.len()
                        );
                    }
                }
            }
            _ => panic!("Expected 2 println! macros, got {}", macros.len()),
        }
    }
}
