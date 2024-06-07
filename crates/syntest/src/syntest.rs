use std::fs;
use std::path::PathBuf;
use syn::{parse_file, Expr, Item, ItemFn, Lit, Local, Pat, PathSegment, Stmt};

pub struct Syntest {
    pub file: syn::File,
}

impl Syntest {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            file: parse_file(&fs::read_to_string(&path)?)?,
        })
    }

    pub fn get_local_value(&self, fn_name: &str, local_name: &str) -> Option<LocalVariable> {
        let mut used = false;
        let mut local_value: Option<LocalVariable> = None;

        self.file.items.iter().for_each(|item| {
            if let Item::Fn(f) = item {
                f.block.stmts.iter().for_each(|stmt| {
                    if let Stmt::Expr(expr, _) = stmt {
                        match expr {
                            Expr::Binary(expr_binary) => {
                                if let Expr::Path(expr_path) = *expr_binary.left.clone() {
                                    expr_path.path.segments.iter().for_each(|path_segment| {
                                        if path_segment.ident == local_name {
                                            used = true;
                                        }
                                    });
                                }
                            }
                            _ => {}
                        }
                    }

                    if let Stmt::Local(local) = stmt {
                        if f.sig.ident == fn_name {
                            if let Pat::Ident(pat) = &local.pat {
                                if pat.ident != local_name {
                                    return;
                                }

                                if let Some(init) = &local.init {
                                    match *init.expr.clone() {
                                        Expr::Lit(expr_lit) => match expr_lit.lit {
                                            Lit::Str(lit_str) => {
                                                local_value = Some(LocalVariable::Str {
                                                    name: pat.ident.to_string(),
                                                    value: lit_str.value(),
                                                    used,
                                                });
                                            }
                                            Lit::Int(lit_int) => {
                                                local_value = Some(LocalVariable::Int {
                                                    name: pat.ident.to_string(),
                                                    value: lit_int.base10_parse().unwrap(),
                                                    used,
                                                });
                                            }
                                            Lit::Float(lit_float) => {
                                                local_value = Some(LocalVariable::Float {
                                                    name: pat.ident.to_string(),
                                                    value: lit_float.base10_parse().unwrap(),
                                                    used,
                                                });
                                            }
                                            Lit::Char(lit_char) => {
                                                local_value = Some(LocalVariable::Char {
                                                    name: pat.ident.to_string(),
                                                    value: lit_char.value(),
                                                    used,
                                                });
                                            }
                                            Lit::Bool(lit_bool) => {
                                                local_value = Some(LocalVariable::Bool {
                                                    name: pat.ident.to_string(),
                                                    value: lit_bool.value(),
                                                    used,
                                                });
                                            }
                                            _ => {}
                                        },
                                        Expr::Closure(_) => {
                                            local_value = Some(LocalVariable::Closure {
                                                name: pat.ident.to_string(),
                                                used,
                                            });
                                        }
                                        _ => {
                                            local_value = Some(LocalVariable::Other {
                                                name: pat.ident.to_string(),
                                                used,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
            }
        });

        local_value
    }

    pub fn expr<F>(&self, mut handler: F)
    where
        F: FnMut(&Expr),
    {
        self.file.items.iter().for_each(|item| {
            let mut ran = false;
            if let Item::Fn(f) = item {
                f.block.stmts.iter().for_each(|stmt| {
                    if let Stmt::Expr(expr, _) = stmt {
                        handler(expr);
                        ran = true;
                    }
                })
            }

            if !ran {
                panic!("No expression found");
            }
        })
    }

    pub fn local<F>(&self, mut handler: F)
    where
        F: FnMut(&Local),
    {
        let mut ran = false;
        self.file.items.iter().for_each(|item| {
            if let Item::Fn(f) = item {
                f.block.stmts.iter().for_each(|stmt| {
                    if let Stmt::Local(local) = stmt {
                        handler(local);
                        ran = true;
                    }
                })
            }
        });

        if !ran {
            panic!("No local found");
        }
    }

    pub fn func<F>(&self, fn_name: &str, mut handler: F)
    where
        F: FnMut(&ItemFn),
    {
        let mut ran = false;
        self.file.items.iter().for_each(|item| {
            if let Item::Fn(f) = item {
                if f.sig.ident == fn_name {
                    handler(f);
                    ran = true;
                }
            }
        });

        if !ran {
            panic!("No function found");
        }
    }

    pub fn func_stmts<F>(&self, fn_name: &str, mut handler: F)
    where
        F: FnMut(&ItemFn, &Stmt),
    {
        let mut ran = false;
        self.func(fn_name, |f| {
            f.block.stmts.iter().for_each(|stmt| {
                handler(f, stmt);
                ran = true;
            })
        });

        if !ran {
            panic!("No function found");
        }
    }

    /// Finds all variables defined in a function block
    /// and checks if they are used or not
    pub fn find_variables(&self, fn_name: &str) -> Vec<LocalVariable> {
        let mut variables = Vec::new();

        self.func_stmts(fn_name, |_, stmt| match stmt {
            Stmt::Local(local) => {
                let used = false;
                let mut local_value = LocalVariable::Other {
                    name: "".to_string(),
                    used: false,
                };

                if let Pat::Ident(ident) = &local.pat {
                    if let Some(init) = &local.init {
                        match *init.expr.clone() {
                            Expr::Lit(expr_lit) => match expr_lit.lit {
                                Lit::Str(lit_str) => {
                                    local_value = LocalVariable::Str {
                                        name: ident.ident.to_string(),
                                        value: lit_str.value(),
                                        used,
                                    };
                                }
                                Lit::Int(lit_int) => {
                                    local_value = LocalVariable::Int {
                                        name: ident.ident.to_string(),
                                        value: lit_int.base10_parse().unwrap(),
                                        used,
                                    };
                                }
                                Lit::Float(lit_float) => {
                                    local_value = LocalVariable::Float {
                                        name: ident.ident.to_string(),
                                        value: lit_float.base10_parse().unwrap(),
                                        used,
                                    };
                                }
                                Lit::Char(lit_char) => {
                                    local_value = LocalVariable::Char {
                                        name: ident.ident.to_string(),
                                        value: lit_char.value(),
                                        used,
                                    };
                                }
                                Lit::Bool(lit_bool) => {
                                    local_value = LocalVariable::Bool {
                                        name: ident.ident.to_string(),
                                        value: lit_bool.value(),
                                        used,
                                    };
                                }
                                _ => {}
                            },
                            Expr::Binary(expr_binary) => {
                                variables.iter_mut().for_each(|variable| {
                                    let mut handle_segment =
                                        |path_segment: &PathSegment| match variable {
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
                                        };

                                    if let Expr::Path(expr_path) = *expr_binary.left.clone() {
                                        expr_path
                                            .path
                                            .segments
                                            .iter()
                                            .for_each(&mut handle_segment);
                                    }
                                    if let Expr::Path(expr_path) = *expr_binary.right.clone() {
                                        expr_path.path.segments.iter().for_each(handle_segment);
                                    }
                                });
                            }
                            Expr::Closure(_) => {
                                local_value = LocalVariable::Closure {
                                    name: ident.ident.to_string(),
                                    used,
                                };
                            }
                            _ => {
                                local_value = LocalVariable::Other {
                                    name: ident.ident.to_string(),
                                    used,
                                };
                            }
                        }
                    }
                }

                variables.push(local_value);
            }
            Stmt::Expr(expr, _) => match expr {
                Expr::Binary(binary_expr) => {
                    let mut check_segment = |path_segment: &PathSegment| {
                        let mut found = false;
                        variables.iter_mut().for_each(|variable| match variable {
                            LocalVariable::Str { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Int { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Float { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Char { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Bool { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Closure { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                            LocalVariable::Other { name, used, .. } => {
                                if name == &path_segment.ident.to_string() {
                                    *used = true;
                                    found = true;
                                }
                            }
                        });

                        if !found {
                            variables.push(LocalVariable::Other {
                                name: path_segment.ident.to_string(),
                                used: true,
                            });
                        }
                    };

                    if let Expr::Path(expr_path) = *binary_expr.left.clone() {
                        expr_path.path.segments.iter().for_each(&mut check_segment);
                    }

                    if let Expr::Path(expr_path) = *binary_expr.right.clone() {
                        expr_path.path.segments.iter().for_each(check_segment);
                    }
                }
                _ => {}
            },

            _ => {}
        });

        variables
    }
}

impl From<&str> for Syntest {
    fn from(path: &str) -> Self {
        Syntest::new(PathBuf::from(path)).unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum LocalVariable {
    Str {
        name: String,
        value: String,
        used: bool,
    },
    Int {
        name: String,
        value: usize,
        used: bool,
    },
    Float {
        name: String,
        value: f64,
        used: bool,
    },
    Char {
        name: String,
        value: char,
        used: bool,
    },
    Bool {
        name: String,
        value: bool,
        used: bool,
    },
    Closure {
        name: String,
        used: bool,
    },
    Other {
        name: String,
        used: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_test_file(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().expect("Unable to create temp file");
        write!(temp_file, "{}", content).expect("Unable to write to temp file");
        temp_file
    }

    #[test]
    fn test_get_local_value_str() {
        let content = r#"
        pub fn test_fn() {
            let my_local_str = "local";
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(
            syntest.get_local_value("test_fn", "my_local_str").unwrap(),
            LocalVariable::Str {
                name: "my_local_str".to_string(),
                value: "local".to_string(),
                used: false
            }
        );
    }

    #[test]
    fn test_get_local_value_int() {
        let content = r#"
        pub fn test_fn() {
            let my_local_int = 42;
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(
            syntest.get_local_value("test_fn", "my_local_int").unwrap(),
            LocalVariable::Int {
                name: "my_local_int".to_string(),
                value: 42,
                used: false
            }
        );
    }

    #[test]
    fn test_get_local_value_float() {
        let content = r#"
        pub fn test_fn() {
            let my_local_float = 3.14;
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(
            syntest
                .get_local_value("test_fn", "my_local_float")
                .unwrap(),
            LocalVariable::Float {
                name: "my_local_float".to_string(),
                value: 3.14,
                used: false
            }
        );
    }

    #[test]
    fn test_get_local_value_char() {
        let content = r#"
        pub fn test_fn() {
            let my_local_char = 'a';
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(
            syntest.get_local_value("test_fn", "my_local_char").unwrap(),
            LocalVariable::Char {
                name: "my_local_char".to_string(),
                value: 'a',
                used: false
            }
        );
    }

    #[test]
    fn test_get_local_value_bool() {
        let content = r#"
        pub fn test_fn() {
            let my_local_bool = true;
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(
            syntest.get_local_value("test_fn", "my_local_bool").unwrap(),
            LocalVariable::Bool {
                name: "my_local_bool".to_string(),
                value: true,
                used: false
            }
        );
    }

    #[test]
    fn test_get_local_value_non_existent_variable() {
        let content = r#"
        pub fn test_fn() {
            let my_local_bool = true;
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert!(syntest
            .get_local_value("test_fn", "non_existent_var")
            .is_none());
    }

    #[test]
    fn test_get_local_value_non_existent_function() {
        let content = r#"
        pub fn test_fn() {
            let my_local_bool = true;
        }
        "#;
        let temp_file = write_temp_test_file(content);
        let syntest = Syntest::new(temp_file.path().to_path_buf()).unwrap();

        assert!(syntest
            .get_local_value("non_existent_fn", "my_local_bool")
            .is_none());
    }
}
