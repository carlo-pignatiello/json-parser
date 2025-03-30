mod tokenize;
mod parse;
use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}
