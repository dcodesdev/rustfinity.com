use std::rc::Rc;
use syn::Stmt;

use crate::func::Func;

/// Helper functions for macros
#[derive(Debug)]
pub struct Mac {
    file: Rc<syn::File>,
}

#[derive(Debug)]
pub struct MacroDetails {
    pub name: String,
    pub tokens: Vec<String>,
}

impl Mac {
    pub fn new(file: Rc<syn::File>) -> Self {
        Self { file }
    }

    pub fn macros(&self, fn_name: &str) -> Vec<MacroDetails> {
        let mut macros = Vec::new();

        self.func_stmts(fn_name, |_, stmt| {
            if let Stmt::Macro(macro_stmt) = stmt {
                let mac = &macro_stmt.mac;

                mac.path.segments.iter().for_each(|seg| {
                    let macro_name = seg.ident.to_string();
                    let tokens = mac
                        .tokens
                        .clone()
                        .into_iter()
                        .map(|token| token.to_string())
                        .collect();

                    macros.push(MacroDetails {
                        name: macro_name,
                        tokens,
                    });
                });
            }
        });

        macros
    }
}

impl Func for Mac {
    fn file(&self) -> &Rc<syn::File> {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use crate::Syntest;

    #[test]
    fn test_list_macros() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;

            let re_assigned = my_local_int + another_local_int * 2 / 2 - 2;

            println!("Result: {}", re_assigned);
            println!("Second macro");

            return re_assigned;
        }
        "#;

        let mac = Syntest::from_code(content).unwrap().mac;

        let macros = mac.macros("test_fn");

        assert_eq!(macros.len(), 2);

        match &macros[..] {
            [first, second] => {
                // Testing the first one
                assert_eq!(first.name, "println");
                first
                    .tokens
                    .iter()
                    .enumerate()
                    .for_each(|(i, token)| match i {
                        0 => assert_eq!(token, "\"Result: {}\""),
                        1 => assert_eq!(token, ","),
                        2 => assert_eq!(token, "re_assigned"),
                        _ => panic!("Unexpected token"),
                    });

                // Testing the second one
                assert_eq!(second.name, "println");
                second
                    .tokens
                    .iter()
                    .enumerate()
                    .for_each(|(i, token)| match i {
                        0 => assert_eq!(token, "\"Second macro\""),
                        _ => panic!("Unexpected token"),
                    });
            }
            _ => panic!("Expected two macros"),
        }
    }
}
