use proc_macro2::TokenTree;
use std::fmt::Display;

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

#[derive(Debug, PartialEq)]
pub enum Value {
    Str(String),
    Int(usize),
    Float(f64),
    Char(char),
    Bool(bool),
    Vec(Vec<Value>),
    Closure,
    Other,
}

impl From<TokenTree> for Value {
    fn from(value: TokenTree) -> Self {
        match value {
            TokenTree::Group(group) => {
                let mut tokens = Vec::new();

                for token in group.stream() {
                    tokens.push(Value::from(token));
                }

                Value::Vec(tokens)
            }
            TokenTree::Ident(ident) => Value::Str(ident.to_string()),
            TokenTree::Literal(lit) => Value::Str(lit.to_string()),
            TokenTree::Punct(punct) => Value::Char(punct.as_char()),
        }
    }
}

impl LocalVariable {
    pub fn is_used(&self) -> bool {
        match self {
            LocalVariable::Str { used, .. } => *used,
            LocalVariable::Int { used, .. } => *used,
            LocalVariable::Float { used, .. } => *used,
            LocalVariable::Char { used, .. } => *used,
            LocalVariable::Bool { used, .. } => *used,
            LocalVariable::Closure { used, .. } => *used,
            LocalVariable::Other { used, .. } => *used,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            LocalVariable::Str { name, .. } => name,
            LocalVariable::Int { name, .. } => name,
            LocalVariable::Float { name, .. } => name,
            LocalVariable::Char { name, .. } => name,
            LocalVariable::Bool { name, .. } => name,
            LocalVariable::Closure { name, .. } => name,
            LocalVariable::Other { name, .. } => name,
        }
    }

    pub fn value(&self) -> Value {
        match self {
            LocalVariable::Str { value, .. } => Value::Str(value.clone()),
            LocalVariable::Int { value, .. } => Value::Int(*value),
            LocalVariable::Float { value, .. } => Value::Float(*value),
            LocalVariable::Char { value, .. } => Value::Char(*value),
            LocalVariable::Bool { value, .. } => Value::Bool(*value),
            LocalVariable::Closure { .. } => Value::Closure,
            LocalVariable::Other { .. } => Value::Other,
        }
    }
}

impl Display for LocalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
