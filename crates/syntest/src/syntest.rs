use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use syn::{parse_file, punctuated::Punctuated, token::PathSep, Expr, Lit, Pat, PathSegment, Stmt};

use crate::{func::Func, mac::Mac, var::LocalVariable};

pub struct Syntest {
    pub file: Rc<syn::File>,
    pub mac: Mac,
}

impl Syntest {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let file = Rc::new(parse_file(&fs::read_to_string(&path)?)?);

        Ok(Self {
            mac: Mac::new(Rc::clone(&file)),
            file,
        })
    }

    pub fn from_code(code: &str) -> anyhow::Result<Self> {
        let file = Rc::new(parse_file(code)?);

        Ok(Self {
            mac: Mac::new(Rc::clone(&file)),
            file: Rc::new(parse_file(code)?),
        })
    }

    /// Finds all variables defined in a function block
    /// and checks if they are used or not
    pub fn variables(&self, fn_name: &str) -> Vec<LocalVariable> {
        let mut variables = Vec::new();

        self.func_stmts(fn_name, |_, stmt| match stmt {
            Stmt::Local(local) => {
                let used = false;
                let mut local_value = None;

                if let Pat::Ident(ident) = &local.pat {
                    let mutable = ident.mutability.is_some();

                    if let Some(init) = &local.init {
                        match *init.expr.clone() {
                            Expr::Lit(expr_lit) => match expr_lit.lit {
                                Lit::Str(lit_str) => {
                                    local_value = Some(LocalVariable::Str {
                                        name: ident.ident.to_string(),
                                        value: lit_str.value(),
                                        used,
                                        mutable,
                                    });
                                }
                                Lit::Int(lit_int) => {
                                    local_value = Some(LocalVariable::Int {
                                        name: ident.ident.to_string(),
                                        value: lit_int.base10_parse().unwrap(),
                                        used,
                                        mutable,
                                    });
                                }
                                Lit::Float(lit_float) => {
                                    local_value = Some(LocalVariable::Float {
                                        name: ident.ident.to_string(),
                                        value: lit_float.base10_parse().unwrap(),
                                        used,
                                        mutable,
                                    });
                                }
                                Lit::Char(lit_char) => {
                                    local_value = Some(LocalVariable::Char {
                                        name: ident.ident.to_string(),
                                        value: lit_char.value(),
                                        used,
                                        mutable,
                                    });
                                }
                                Lit::Bool(lit_bool) => {
                                    local_value = Some(LocalVariable::Bool {
                                        name: ident.ident.to_string(),
                                        value: lit_bool.value(),
                                        used,
                                        mutable,
                                    });
                                }
                                _ => {}
                            },
                            Expr::Closure(_) => {
                                local_value = Some(LocalVariable::Closure {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                });
                            }
                            Expr::Binary(expr_binary) => {
                                self.match_expr(&expr_binary.left, &mut variables);
                                self.match_expr(&expr_binary.right, &mut variables);

                                local_value = Some(LocalVariable::Other {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                })
                            }
                            Expr::Path(expr_path) => {
                                self.create_var_from_segments(
                                    &mut variables,
                                    &expr_path.path.segments,
                                    mutable,
                                );
                            }
                            _ => {
                                local_value = Some(LocalVariable::Other {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                });
                            }
                        }
                    }
                }

                if let Some(local_value) = local_value {
                    variables.push(local_value);
                }
            }
            Stmt::Expr(expr, _) => self.match_expr(expr, &mut variables),
            _ => {}
        });

        variables
    }

    fn match_expr(&self, expr: &Expr, variables: &mut Vec<LocalVariable>) {
        match expr {
            Expr::Binary(binary_expr) => {
                self.match_expr(&binary_expr.left, variables);
                self.match_expr(&binary_expr.right, variables);
            }
            Expr::Path(path_expr) => {
                self.match_segments(variables, &path_expr.path.segments);
            }
            Expr::Return(return_expr) => {
                if let Some(expr) = return_expr.expr.clone() {
                    self.match_expr(&expr, variables)
                }
            }
            _ => {}
        }
    }

    fn create_var_from_segments(
        &self,
        variables: &mut Vec<LocalVariable>,
        segments: &Punctuated<PathSegment, PathSep>,
        mutable: bool,
    ) {
        segments.iter().for_each(|segment| {
            variables.push(LocalVariable::Other {
                name: segment.ident.to_string(),
                used: false,
                mutable,
            })
        });
    }

    fn match_segments(
        &self,
        variables: &mut Vec<LocalVariable>,
        segments: &Punctuated<PathSegment, PathSep>,
    ) {
        let mut check_segment = |path_segment: &PathSegment| {
            variables.iter_mut().for_each(|variable| match variable {
                LocalVariable::Str { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Int { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Float { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Char { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Bool { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Closure { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
                LocalVariable::Other { name, used, .. } => {
                    if name == &path_segment.ident.to_string() {
                        *used = true;
                    }
                }
            });
        };

        segments.iter().for_each(&mut check_segment);
    }

    pub fn var_details(&self, fn_name: &str, var_name: &str) -> Option<LocalVariable> {
        let vars = self.variables(fn_name);

        vars.into_iter().find(|var| var.name() == var_name)
    }

    /// Counts how many times a variable was declared
    pub fn count_var(&self, fn_name: &str, var_name: &str) -> usize {
        let vars = self.variables(fn_name);

        vars.into_iter()
            .filter(|var| var.name() == var_name)
            .count()
    }
}

impl From<&str> for Syntest {
    fn from(path: &str) -> Self {
        Syntest::new(PathBuf::from(path)).unwrap()
    }
}

impl Func for Syntest {
    fn file(&self) -> &Rc<syn::File> {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_value_str() {
        let content: &str = r#"
        pub fn test_fn() {
            let my_local_str = "local";
        }
        "#;
        let vars = Syntest::from_code(content).unwrap().variables("test_fn");

        vars.iter().for_each(|var| match var {
            LocalVariable::Str {
                name,
                value,
                used,
                mutable,
            } => {
                assert_eq!(name, "my_local_str");
                assert_eq!(value, "local");
                assert_eq!(*used, false);
                assert_eq!(*mutable, false);
            }
            _ => {
                panic!("Invalid variable type")
            }
        })
    }

    #[test]
    fn test_multiple_values() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;
            let mut local_str = "string";

            let re_assigned = my_local_int + another_local_int;
        }
        "#;

        let vars = Syntest::from_code(content).unwrap().variables("test_fn");

        vars.iter().for_each(|var| match var {
            LocalVariable::Int {
                name,
                value,
                used,
                mutable,
            } => match name.as_str() {
                "my_local_int" => {
                    assert_eq!(*value, 42);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, false);
                }
                "another_local_int" => {
                    assert_eq!(*value, 10);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, false);
                }
                _ => {
                    panic!("Invalid variable name")
                }
            },
            LocalVariable::Str {
                name,
                value,
                used,
                mutable,
            } => {
                assert_eq!(name, "local_str");
                assert_eq!(value, "string");
                assert_eq!(*used, false);
                assert_eq!(*mutable, true);
            }
            LocalVariable::Other {
                name,
                used,
                mutable,
            } => {
                assert_eq!(name, "re_assigned");
                assert_eq!(*used, false);
                assert_eq!(*mutable, false);
            }
            _ => {
                panic!("Invalid variable type")
            }
        })
    }

    #[test]
    fn test_return_tail() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;
            let local_str = "string";

            let mut re_assigned = my_local_int + another_local_int;

            re_assigned
        }
        "#;

        let vars = Syntest::from_code(content).unwrap().variables("test_fn");

        let mut asserted = false;

        vars.iter().for_each(|var| match var {
            LocalVariable::Other {
                name,
                used,
                mutable,
            } => {
                assert_eq!(name, "re_assigned");
                assert!(*used);
                assert!(mutable);
                asserted = true;
            }
            _ => {}
        });

        if !asserted {
            panic!("No return value found");
        }
    }

    #[test]
    fn test_return_keyword() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;
            let local_str = "string";

            let re_assigned = my_local_int + another_local_int;

            return re_assigned;
        }
        "#;

        let vars = Syntest::from_code(content).unwrap().variables("test_fn");

        let mut asserted = false;

        vars.iter().for_each(|var| match var {
            LocalVariable::Other {
                name,
                used,
                mutable,
            } => {
                assert_eq!(name, "re_assigned");
                assert_eq!(*used, true);
                assert_eq!(*mutable, false);
                asserted = true;
            }
            _ => {}
        });

        if !asserted {
            panic!("No return value found");
        }
    }

    #[test]
    #[should_panic(expected = "Function non_existent_fn not found")]
    fn test_get_non_existent_function() {
        let content = r#"
        pub fn test_fn() {
            let my_local_bool = true;
        }
        "#;

        Syntest::from_code(content)
            .unwrap()
            .variables("non_existent_fn");
    }

    #[test]
    fn test_var_details() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;
            let local_str = "string";

            let re_assigned = my_local_int + another_local_int;

            return re_assigned;
        }
        "#;

        let syntest = Syntest::from_code(content).unwrap();

        let vars = [
            "my_local_int",
            "another_local_int",
            "local_str",
            "re_assigned",
        ];

        vars.iter().for_each(|&var| {
            let details = syntest.var_details("test_fn", var).unwrap();

            if var == "local_str" {
                assert_eq!(details.is_used(), false);
            } else {
                assert_eq!(details.is_used(), true);
            }

            assert_eq!(details.name(), var);
        })
    }

    #[test]
    fn test_multiple_math_ops() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;
            let local_str = "string";

            let re_assigned = my_local_int + another_local_int;
            let re_assigned2 = re_assigned * 2;
            let re_assigned3 = re_assigned2 / 2;
            let re_assigned4 = re_assigned3 - 2;

            return re_assigned4;
        }
        "#;

        let syntest = Syntest::from_code(content).unwrap();

        let vars = [
            "my_local_int",
            "another_local_int",
            "local_str",
            "re_assigned",
            "re_assigned2",
            "re_assigned3",
            "re_assigned4",
        ];

        vars.iter().for_each(|&var| {
            let details = syntest.var_details("test_fn", var).unwrap();

            if var == "local_str" {
                assert_eq!(details.is_used(), false);
            } else {
                assert_eq!(details.is_used(), true);
            }

            assert_eq!(details.name(), var, "Variable name mismatch");
        })
    }

    #[test]
    fn test_multi_op_one_line() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;

            let re_assigned = my_local_int + another_local_int * 2 / 2 - 2;

            return re_assigned;
        }
        "#;
        let syntest = Syntest::from_code(content).unwrap();
        let vars = ["my_local_int", "another_local_int", "re_assigned"];

        vars.iter().for_each(|&var| {
            let details = syntest.var_details("test_fn", var).unwrap();

            assert!(details.is_used(), "Variable {} not used", var);
            assert_eq!(details.name(), var, "Variable name mismatch");
        })
    }

    #[test]
    fn test_var_count() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
            let another_local_int = 10;

            let re_assigned = my_local_int + another_local_int;

            let width = 10;
            let width = 20;

            let height = 5;
            let height = 10;
            let height = 15;

            return re_assigned;
        }
        "#;

        let syntest = Syntest::from_code(content).unwrap();

        assert_eq!(syntest.count_var("test_fn", "my_local_int"), 1);
        assert_eq!(syntest.count_var("test_fn", "another_local_int"), 1);
        assert_eq!(syntest.count_var("test_fn", "re_assigned"), 1);
        assert_eq!(syntest.count_var("test_fn", "width"), 2);
        assert_eq!(syntest.count_var("test_fn", "height"), 3);
    }
}
