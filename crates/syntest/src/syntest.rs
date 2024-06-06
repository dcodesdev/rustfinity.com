use std::fs;
use std::path::PathBuf;
use syn::{parse_file, Expr, Item, Lit, Pat, Stmt};

pub struct Syntest {
    file: syn::File,
}

impl Syntest {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            file: parse_file(&fs::read_to_string(&path)?)?,
        })
    }

    pub fn get_local_value(&self, fn_name: &str, local_name: &str) -> Option<LocalValue> {
        self.file.items.iter().find_map(|item| {
            if let Item::Fn(f) = item {
                if f.sig.ident != fn_name {
                    return None;
                }

                f.block.stmts.iter().find_map(|stmt| {
                    if let Stmt::Local(local) = stmt {
                        if let Pat::Ident(pat) = &local.pat {
                            if pat.ident != local_name {
                                return None;
                            }

                            if let Some(init) = &local.init {
                                match *init.expr.clone() {
                                    Expr::Lit(expr_lit) => match expr_lit.lit {
                                        Lit::Str(lit_str) => {
                                            return Some(LocalValue::Str(lit_str.value()));
                                        }
                                        Lit::Int(lit_int) => {
                                            return Some(LocalValue::Int(
                                                lit_int.base10_parse().unwrap(),
                                            ));
                                        }
                                        Lit::Float(lit_float) => {
                                            return Some(LocalValue::Float(
                                                lit_float.base10_parse().unwrap(),
                                            ));
                                        }
                                        Lit::Char(lit_char) => {
                                            return Some(LocalValue::Char(lit_char.value()));
                                        }
                                        Lit::Bool(lit_bool) => {
                                            return Some(LocalValue::Bool(lit_bool.value()));
                                        }
                                        _ => {
                                            return None;
                                        }
                                    },
                                    Expr::Closure(_) => {
                                        return Some(LocalValue::Closure);
                                    }
                                    _ => {
                                        return Some(LocalValue::Other);
                                    }
                                }
                            }
                        }
                    }
                    None
                })
            } else {
                None
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum LocalValue {
    Str(String),
    Int(usize),
    Float(f64),
    Char(char),
    Bool(bool),
    Closure,
    Other,
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
        let res = syntest.get_local_value("test_fn", "my_local_str").unwrap();
        let expected_value = LocalValue::Str("local".to_string());

        assert_eq!(res, expected_value);
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
        let res = syntest.get_local_value("test_fn", "my_local_int").unwrap();
        let expected_value = LocalValue::Int(42);

        assert_eq!(res, expected_value);
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
        let res = syntest
            .get_local_value("test_fn", "my_local_float")
            .unwrap();
        let expected_value = LocalValue::Float(3.14);

        assert_eq!(res, expected_value);
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
        let res = syntest.get_local_value("test_fn", "my_local_char").unwrap();
        let expected_value = LocalValue::Char('a');

        assert_eq!(res, expected_value);
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
        let res = syntest.get_local_value("test_fn", "my_local_bool").unwrap();
        let expected_value = LocalValue::Bool(true);

        assert_eq!(res, expected_value);
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
        let res = syntest.get_local_value("test_fn", "non_existent_var");

        assert!(res.is_none());
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
        let res = syntest.get_local_value("non_existent_fn", "my_local_bool");

        assert!(res.is_none());
    }
}
