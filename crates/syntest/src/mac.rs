use std::rc::Rc;
use syn::{Expr, Macro, Stmt};

use crate::func::Func;

/// Helper functions for macros
#[derive(Debug)]
pub struct Mac {
    file: Rc<syn::File>,
    fn_name: String,
}

#[derive(Debug)]
pub struct MacroDetails {
    pub name: String,
    pub tokens: Vec<String>,
}

impl Mac {
    pub fn new(fn_name: &str, file: Rc<syn::File>) -> Self {
        Self {
            file,
            fn_name: fn_name.to_string(),
        }
    }

    pub fn macros(&self) -> Vec<MacroDetails> {
        let mut macros = Vec::new();

        let mut handle_macro_expr = |mac: &Macro| {
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
        };

        self.func_stmts(|_, stmt| match stmt {
            Stmt::Macro(macro_stmt) => {
                handle_macro_expr(&macro_stmt.mac);
            }
            Stmt::Expr(expr, _) => {
                if let Expr::Macro(macro_expr) = expr {
                    handle_macro_expr(&macro_expr.mac);
                }
            }
            _ => {}
        });

        macros
    }
}

impl Func for Mac {
    fn file(&self) -> &Rc<syn::File> {
        &self.file
    }

    fn fn_name(&self) -> &str {
        &self.fn_name
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

        let mac = Syntest::from_code("test_fn", content).mac;
        let macros = mac.macros();

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
