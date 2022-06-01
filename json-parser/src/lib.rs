use std::{collections::BTreeMap, string::ParseError};

mod lexer;
mod parser;

pub use lexer::*;
pub use parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

pub fn parse(input: &str) -> Result<Value, ParseError> {
    todo!()
}

impl std::ops::Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Value::Object(map) => map
                .get(key)
                .unwrap_or_else(|| panic!("A key is not found: {}", key)),
            _ => {
                panic!("A value is not object");
            }
        }
    }
}

impl std::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(array) => &array[index],
            _ => {
                panic!("A value is not array");
            }
        }
    }
    
}
