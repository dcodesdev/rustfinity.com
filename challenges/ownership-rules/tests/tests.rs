#[cfg(test)]
mod tests {
    use std::fs;

    use ownership_rules::calculate_and_modify;
    use syntest::{visit::Visit, ExprMethodCall, Syntest};

    #[derive(Debug)]
    struct Visitor {
        push_str_calls: Vec<ExprMethodCall>,
    }

    impl Visitor {
        fn new(fn_name: &str, code: &str) -> Self {
            let file = syntest::parse_file(code).expect("Failed to parse file");

            let mut visitor = Self {
                push_str_calls: Vec::new(),
            };

            let items = file
                .items
                .iter()
                .find_map(|item| {
                    if let syntest::Item::Fn(fn_item) = item {
                        if fn_item.sig.ident == fn_name {
                            Some(fn_item)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .expect(&format!("Function {} not found", fn_name));

            visitor.visit_item_fn(items);

            visitor
        }
    }

    impl<'ast> Visit<'ast> for Visitor {
        fn visit_expr_method_call(&mut self, method_call: &'ast ExprMethodCall) {
            if method_call.method == "push_str" {
                self.push_str_calls.push(method_call.clone());
            }
        }
    }

    #[test]
    fn test_calculate_and_modify() {
        let (s, length) = calculate_and_modify();
        assert_eq!(s, "hello, world");
        assert_eq!(length, 5);
    }

    #[test]
    fn test_ownership_violations() {
        {
            let code = fs::read_to_string("src/lib.rs").expect("Failed to read file");
            let visitor = Visitor::new("calculate_and_modify", &code);

            assert_eq!(
                visitor.push_str_calls.len(),
                1,
                "Expected push_str to be called"
            );

            // check the macro to be used `println!`
            let syntest = Syntest::new("calculate_and_modify", "src/lib.rs");
            let macros = syntest.mac.macros();
            let macro_call = &macros[0];

            assert_eq!(macros.len(), 1, "Expected println! to be called");
            assert_eq!(macro_call.name, "println", "Expected println! to be called");
            assert_eq!(
                macro_call.tokens.get(0).unwrap(),
                "\"{}\"",
                "Expected println! to be called"
            );
            assert_eq!(
                macro_call.tokens.get(2).unwrap(),
                "s2",
                "Expected println! to be called with s2"
            );
        }

        // Local tests outside users code
        {
            let code = r#"
            pub fn calculate_and_modify() -> (String, usize) {
                let mut s = String::from("hello");
                let length = s.len();
                s.push_str(", world");
                (s, length)
            }"#;

            let visitor = Visitor::new("calculate_and_modify", code);

            assert_eq!(
                visitor.push_str_calls.len(),
                1,
                "Expected push_str to be called"
            );
        }
    }
}
