use crate::Value;

#[derive(Debug, PartialEq)]
pub struct Mutation {
    from: Value,
    to: Value,
}

impl Mutation {
    pub fn new(from: Value, to: Value) -> Self {
        Self { from, to }
    }
}
