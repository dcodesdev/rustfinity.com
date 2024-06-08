use proc_macro2::TokenTree;
use std::fmt::Display;

use crate::mutation::Mutation;

#[derive(Debug, PartialEq)]
pub enum LocalVariable {
    Str {
        name: String,
        value: String,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Int {
        name: String,
        value: usize,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Float {
        name: String,
        value: f64,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Char {
        name: String,
        value: char,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Bool {
        name: String,
        value: bool,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Closure {
        name: String,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
    Other {
        name: String,
        used: bool,
        mutable: bool,
        mutations: Vec<Mutation>,
    },
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

    pub fn is_mutable(&self) -> bool {
        match self {
            LocalVariable::Str { mutable, .. } => *mutable,
            LocalVariable::Int { mutable, .. } => *mutable,
            LocalVariable::Float { mutable, .. } => *mutable,
            LocalVariable::Char { mutable, .. } => *mutable,
            LocalVariable::Bool { mutable, .. } => *mutable,
            LocalVariable::Closure { mutable, .. } => *mutable,
            LocalVariable::Other { mutable, .. } => *mutable,
        }
    }

    pub fn set_to_used(&mut self) {
        match self {
            LocalVariable::Str { used, .. } => *used = true,
            LocalVariable::Int { used, .. } => *used = true,
            LocalVariable::Float { used, .. } => *used = true,
            LocalVariable::Char { used, .. } => *used = true,
            LocalVariable::Bool { used, .. } => *used = true,
            LocalVariable::Closure { used, .. } => *used = true,
            LocalVariable::Other { used, .. } => *used = true,
        }
    }

    pub fn update_value(&mut self, new_value: Value) {
        match self {
            LocalVariable::Str {
                value, mutations, ..
            } => {
                if let Value::Str(new_value) = new_value {
                    mutations.push(Mutation::new(
                        Value::Str(value.clone()),
                        Value::Str(new_value.clone()),
                    ));
                    *value = new_value;
                }
            }
            LocalVariable::Int {
                value, mutations, ..
            } => {
                if let Value::Int(new_value) = new_value {
                    mutations.push(Mutation::new(Value::Int(*value), Value::Int(new_value)));
                    *value = new_value;
                }
            }
            LocalVariable::Float {
                value, mutations, ..
            } => {
                if let Value::Float(new_value) = new_value {
                    mutations.push(Mutation::new(Value::Float(*value), Value::Float(new_value)));
                    *value = new_value;
                }
            }
            LocalVariable::Char {
                value, mutations, ..
            } => {
                if let Value::Char(new_value) = new_value {
                    mutations.push(Mutation::new(Value::Char(*value), Value::Char(new_value)));
                    *value = new_value;
                }
            }
            LocalVariable::Bool {
                value, mutations, ..
            } => {
                if let Value::Bool(new_value) = new_value {
                    mutations.push(Mutation::new(Value::Bool(*value), Value::Bool(new_value)));
                    *value = new_value;
                }
            }
            LocalVariable::Closure { mutations, .. } => {
                mutations.push(Mutation::new(Value::Closure, new_value));
            }
            LocalVariable::Other { mutations, .. } => {
                mutations.push(Mutation::new(Value::Other, new_value));
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl Display for LocalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
