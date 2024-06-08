use std::rc::Rc;
use syn::Stmt;

use crate::func::Func;

/// Helper functions for macros
#[derive(Debug)]
pub struct Mac {
    file: Rc<syn::File>,
}

impl Mac {
    pub fn new(file: Rc<syn::File>) -> Self {
        Self { file }
    }

    pub fn check_println_usage(&self, fn_name: &str) -> bool {
        let mut found = false;
        self.func_stmts(fn_name, |_, stmt| {
            if let Stmt::Macro(macro_stmt) = stmt {
                let mac = &macro_stmt.mac;

                if mac.path.segments.iter().any(|seg| seg.ident == "println") {
                    found = true;
                }
            }
        });

        found
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
    fn test_macro_usage() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;

            let re_assigned = my_local_int + another_local_int * 2 / 2 - 2;

            println!("Result: {}", re_assigned);

            return re_assigned;
        }
        "#;

        let mac = Syntest::from_code(content).unwrap().mac;

        assert!(mac.check_println_usage("test_fn"));
    }
}
