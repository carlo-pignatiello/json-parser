mod tokenize;
use std::collections::HashMap;
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}
