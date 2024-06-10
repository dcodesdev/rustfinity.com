use std::fs;
use std::rc::Rc;
use syn::{
    parse_file, punctuated::Punctuated, token::PathSep, BinOp, Expr, Lit, Pat, PathSegment, Stmt,
};

use crate::{constants::Const, func::Func, mac::Mac, var::LocalVariable, AsVisitor, Value};

pub struct Syntest {
    pub mac: Mac,
    pub constant: Const,
    pub file: Rc<syn::File>,
    pub as_visitor: AsVisitor,
    pub fn_name: String,
}

impl Syntest {
    pub fn new(fn_name: &str, path: &str) -> Self {
        let file = Rc::new(
            parse_file(&fs::read_to_string(&path).expect("Failed to read file"))
                .expect("Failed to parse file"),
        );

        Self {
            mac: Mac::new(fn_name, Rc::clone(&file)),
            constant: Const::new(Rc::clone(&file)),
            as_visitor: AsVisitor::new(fn_name, &file),
            file,
            fn_name: fn_name.to_string(),
        }
    }

    pub fn from_code(fn_name: &str, code: &str) -> Self {
        let file = Rc::new(parse_file(code).expect("Failed to parse file"));

        Self {
            mac: Mac::new(fn_name, Rc::clone(&file)),
            constant: Const::new(Rc::clone(&file)),
            as_visitor: AsVisitor::new(fn_name, &file),
            file,
            fn_name: fn_name.to_string(),
        }
    }

    /// Finds all variables defined in a function block
    /// and checks if they are used or not
    pub fn variables(&self) -> Vec<LocalVariable> {
        let mut variables = Vec::new();

        self.func_stmts(|_, stmt| match stmt {
            Stmt::Local(local) => {
                let used = false;
                let mut new_value = None;

                if let Pat::Ident(ident) = &local.pat {
                    let mutable = ident.mutability.is_some();

                    if let Some(init) = &local.init {
                        match *init.expr.clone() {
                            Expr::Lit(expr_lit) => match expr_lit.lit {
                                Lit::Str(lit_str) => {
                                    new_value = Some(LocalVariable::Str {
                                        name: ident.ident.to_string(),
                                        value: lit_str.value(),
                                        used,
                                        mutable,
                                        mutations: Vec::new(),
                                    });
                                }
                                Lit::Int(lit_int) => {
                                    new_value = Some(LocalVariable::Int {
                                        name: ident.ident.to_string(),
                                        value: lit_int.base10_parse().unwrap(),
                                        used,
                                        mutable,
                                        mutations: Vec::new(),
                                    });
                                }
                                Lit::Float(lit_float) => {
                                    new_value = Some(LocalVariable::Float {
                                        name: ident.ident.to_string(),
                                        value: lit_float.base10_parse().unwrap(),
                                        used,
                                        mutable,
                                        mutations: Vec::new(),
                                    });
                                }
                                Lit::Char(lit_char) => {
                                    new_value = Some(LocalVariable::Char {
                                        name: ident.ident.to_string(),
                                        value: lit_char.value(),
                                        used,
                                        mutable,
                                        mutations: Vec::new(),
                                    });
                                }
                                Lit::Bool(lit_bool) => {
                                    new_value = Some(LocalVariable::Bool {
                                        name: ident.ident.to_string(),
                                        value: lit_bool.value(),
                                        used,
                                        mutable,
                                        mutations: Vec::new(),
                                    });
                                }
                                _ => {}
                            },
                            Expr::Closure(_) => {
                                new_value = Some(LocalVariable::Closure {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                    mutations: Vec::new(),
                                });
                            }
                            Expr::Binary(expr_binary) => {
                                self.match_expr(&expr_binary.left, &mut variables);
                                self.match_expr(&expr_binary.right, &mut variables);

                                new_value = Some(LocalVariable::Other {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                    mutations: Vec::new(),
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
                                new_value = Some(LocalVariable::Other {
                                    name: ident.ident.to_string(),
                                    used,
                                    mutable,
                                    mutations: Vec::new(),
                                });
                            }
                        }
                    }
                }

                if let Some(local_value) = new_value {
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

                if let Expr::Lit(expr_lit) = &*binary_expr.right {
                    if let Expr::Path(left_expr) = &*binary_expr.left {
                        let segments = &left_expr.path.segments;
                        let ident = segments.iter().last().unwrap().ident.to_string();

                        self.update_value_by_op(
                            variables,
                            &ident,
                            &binary_expr.op,
                            self.lit_to_usize(&expr_lit.lit),
                        );
                    }
                };
            }
            Expr::Path(path_expr) => {
                self.set_used_by_segments(variables, &path_expr.path.segments);
            }
            Expr::Return(return_expr) => {
                if let Some(expr) = return_expr.expr.clone() {
                    self.match_expr(&expr, variables)
                }
            }
            Expr::Assign(assign_expr) => match &*assign_expr.left {
                Expr::Path(path_expr) => {
                    let segments = &path_expr.path.segments;
                    let ident = segments.iter().last().unwrap().ident.to_string();

                    match &*assign_expr.right {
                        Expr::Lit(lit) => self.replace_value_by_lit(&lit.lit, variables, &ident),
                        Expr::Path(path_expr) => {
                            self.set_used_by_segments(variables, &path_expr.path.segments)
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn replace_value_by_lit(&self, lit: &Lit, variables: &mut Vec<LocalVariable>, ident: &str) {
        variables.iter_mut().for_each(|var| {
            if ident != var.name() {
                return;
            };

            match lit {
                Lit::Int(lit_int) => {
                    let new_value = lit_int.base10_parse().unwrap();
                    var.replace_value(Value::Int(new_value))
                }
                Lit::Str(lit_str) => {
                    let new_value = lit_str.value();
                    var.replace_value(Value::Str(new_value))
                }
                Lit::Float(lit_float) => {
                    let new_value = lit_float.base10_parse().unwrap();
                    var.replace_value(Value::Float(new_value))
                }
                Lit::Char(lit_char) => {
                    let new_value = lit_char.value();
                    var.replace_value(Value::Char(new_value))
                }
                Lit::Bool(lit_bool) => {
                    let new_value = lit_bool.value();
                    var.replace_value(Value::Bool(new_value))
                }
                _ => {}
            }
        });
    }

    fn update_value_by_op(
        &self,
        variables: &mut Vec<LocalVariable>,
        ident: &str,
        op: &BinOp,
        num: usize,
    ) {
        variables.iter_mut().for_each(|var| {
            if ident != var.name() {
                return;
            };

            match op {
                BinOp::AddAssign(_) => {
                    if let Value::Int(value) = var.value() {
                        var.replace_value(Value::Int(value + num));
                    }
                }
                BinOp::SubAssign(_) => {
                    if let Value::Int(value) = var.value() {
                        var.replace_value(Value::Int(value - num));
                    }
                }
                BinOp::MulAssign(_) => {
                    if let Value::Int(value) = var.value() {
                        var.replace_value(Value::Int(value * num));
                    }
                }
                BinOp::DivAssign(_) => {
                    if let Value::Int(value) = var.value() {
                        var.replace_value(Value::Int(value / num));
                    }
                }
                _ => {}
            }
        });
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
                mutations: Vec::new(),
            })
        });
    }

    fn set_used_by_segments(
        &self,
        variables: &mut Vec<LocalVariable>,
        segments: &Punctuated<PathSegment, PathSep>,
    ) {
        let mut check_segment = |path_segment: &PathSegment| {
            variables.iter_mut().for_each(|variable| {
                if variable.name() == &path_segment.ident.to_string() {
                    variable.set_to_used()
                }
            });
        };

        segments.iter().for_each(&mut check_segment);
    }

    pub fn var_details(&self, var_name: &str) -> Option<LocalVariable> {
        let vars = self.variables();

        vars.into_iter().find(|var| var.name() == var_name)
    }

    /// Counts how many times a variable was declared
    pub fn count_var(&self, var_name: &str) -> usize {
        let vars = self.variables();

        vars.into_iter()
            .filter(|var| var.name() == var_name)
            .count()
    }

    fn lit_to_usize(&self, lit: &Lit) -> usize {
        match lit {
            Lit::Int(lit_int) => lit_int.base10_parse().unwrap(),
            _ => 0,
        }
    }

    pub fn get_stmts(&self) -> Vec<Stmt> {
        let mut locals = Vec::new();

        self.func_stmts(|_, stmt| locals.push(stmt.clone()));

        locals
    }

    pub fn get_pats(&self) -> Vec<Pat> {
        let mut locals = Vec::new();

        self.func_stmts(|_, stmt| {
            if let Stmt::Local(local) = stmt {
                locals.push(local.pat.clone());
            }
        });

        locals
    }
}

impl Func for Syntest {
    fn file(&self) -> &Rc<syn::File> {
        &self.file
    }

    fn fn_name(&self) -> &str {
        &self.fn_name
    }
}

#[cfg(test)]
mod tests {
    use crate::mutation::Mutation;

    use super::*;

    #[test]
    fn test_get_local_value_str() {
        let content: &str = r#"
        pub fn test_fn() {
            let my_local_str = "local";
        }
        "#;
        let vars = Syntest::from_code("test_fn", content).variables();

        vars.iter().for_each(|var| match var {
            LocalVariable::Str {
                name,
                value,
                used,
                mutable,
                mutations,
            } => {
                assert_eq!(name, "my_local_str");
                assert_eq!(value, "local");
                assert_eq!(*used, false);
                assert_eq!(*mutable, false);
                assert_eq!(mutations, &vec![]);
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

            let mut mutating_variable = "something";

            let re_assigned = my_local_int + another_local_int;

            mutating_variable = "something else";
        }
        "#;

        let vars = Syntest::from_code("test_fn", content).variables();

        vars.iter().for_each(|var| match var {
            LocalVariable::Int {
                name,
                value,
                used,
                mutable,
                mutations,
            } => match name.as_str() {
                "my_local_int" => {
                    assert_eq!(*value, 42);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, false);
                    assert_eq!(mutations, &vec![]);
                }
                "another_local_int" => {
                    assert_eq!(*value, 10);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, false);
                    assert_eq!(mutations, &vec![]);
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
                mutations,
            } => match name.as_str() {
                "local_str" => {
                    assert_eq!(name, "local_str");
                    assert_eq!(value, "string");
                    assert_eq!(*used, false);
                    assert_eq!(*mutable, true);
                }
                "mutating_variable" => {
                    assert_eq!(name, "mutating_variable");
                    assert_eq!(value, "something else");
                    assert_eq!(*used, false);
                    assert_eq!(*mutable, true);
                    assert_eq!(
                        mutations,
                        &vec![Mutation::new(
                            Value::Str("something".to_string()),
                            Value::Str("something else".to_string())
                        )]
                    );
                }
                _ => {
                    panic!("Invalid variable name")
                }
            },
            LocalVariable::Other {
                name,
                used,
                mutable,
                mutations,
            } => {
                assert_eq!(name, "re_assigned");
                assert_eq!(*used, false);
                assert_eq!(*mutable, false);
                assert_eq!(mutations, &vec![])
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

        let vars = Syntest::from_code("test_fn", content).variables();

        let mut asserted = false;

        vars.iter().for_each(|var| match var {
            LocalVariable::Other {
                name,
                used,
                mutable,
                mutations,
            } => {
                assert_eq!(name, "re_assigned");
                assert!(*used);
                assert!(mutable);
                assert_eq!(mutations, &vec![]);
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

        let vars = Syntest::from_code("test_fn", content).variables();

        let mut asserted = false;

        vars.iter().for_each(|var| match var {
            LocalVariable::Other {
                name,
                used,
                mutable,
                mutations,
            } => {
                assert_eq!(name, "re_assigned");
                assert_eq!(*used, true);
                assert_eq!(*mutable, false);
                assert_eq!(mutations, &vec![]);
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

        Syntest::from_code("non_existent_fn", content).variables();
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

        let syntest = Syntest::from_code("test_fn", content);

        let vars = [
            "my_local_int",
            "another_local_int",
            "local_str",
            "re_assigned",
        ];

        vars.iter().for_each(|&var| {
            let details = syntest.var_details(var).unwrap();

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

        let syntest = Syntest::from_code("test_fn", content);

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
            let details = syntest.var_details(var).unwrap();

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
        let syntest = Syntest::from_code("test_fn", content);
        let vars = ["my_local_int", "another_local_int", "re_assigned"];

        vars.iter().for_each(|&var| {
            let details = syntest.var_details(var).unwrap();

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

        let syntest = Syntest::from_code("test_fn", content);

        assert_eq!(syntest.count_var("my_local_int"), 1);
        assert_eq!(syntest.count_var("another_local_int"), 1);
        assert_eq!(syntest.count_var("re_assigned"), 1);
        assert_eq!(syntest.count_var("width"), 2);
        assert_eq!(syntest.count_var("height"), 3);
    }

    #[test]
    fn mutating_variables() {
        let content = r#"
        pub fn test_fn() {
            let mut mutable_var = 10;
            mutable_var = 20;
            mutable_var = 30;

            let result = mutable_var + 10;
        }
        "#;

        let syntest = Syntest::from_code("test_fn", content);

        let vars = syntest.variables();

        vars.iter().for_each(|var| match var {
            LocalVariable::Int {
                name,
                value,
                used,
                mutable,
                mutations,
            } => match name.as_str() {
                "mutable_var" => {
                    assert_eq!(*value, 30);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, true);
                    assert_eq!(
                        mutations,
                        &vec![
                            Mutation::new(Value::Int(10), Value::Int(20)),
                            Mutation::new(Value::Int(20), Value::Int(30))
                        ]
                    );
                }
                _ => {
                    panic!("Invalid variable name")
                }
            },
            LocalVariable::Other {
                name,
                used,
                mutable,
                mutations,
            } => {
                assert_eq!(name, "result");
                assert_eq!(*used, false);
                assert_eq!(*mutable, false);
                assert_eq!(mutations, &vec![]);
            }
            _ => {
                panic!("Invalid variable type")
            }
        });
    }

    #[test]
    pub fn test_op_equals() {
        let content = r#"
        pub fn test_fn() {
            let mut mutable_var = 10;
            mutable_var += 10;
            mutable_var -= 5;
            mutable_var *= 2;
            mutable_var /= 2;
        }
        "#;

        let syntest = Syntest::from_code("test_fn", content);

        let vars = syntest.variables();

        vars.iter().for_each(|var| match var {
            LocalVariable::Int {
                name,
                value,
                used,
                mutable,
                mutations,
            } => match name.as_str() {
                "mutable_var" => {
                    assert_eq!(*value, 15);
                    assert_eq!(*used, true);
                    assert_eq!(*mutable, true);
                    assert_eq!(
                        mutations,
                        &vec![
                            Mutation::new(Value::Int(10), Value::Int(20)),
                            Mutation::new(Value::Int(20), Value::Int(15)),
                            Mutation::new(Value::Int(15), Value::Int(30)),
                            Mutation::new(Value::Int(30), Value::Int(15))
                        ]
                    );
                }
                _ => {
                    panic!("Invalid variable name")
                }
            },
            _ => {
                panic!("Invalid variable type")
            }
        });
    }
}
