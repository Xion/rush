use std::collections::hash_map::HashMap;
use std::str;

use nom::{alphanumeric, IResult};


// TODO(xion): make an ADT or something, to represent various types of values
pub type Value = String;
pub type Context = HashMap<String, Value>;

pub trait AstNode {
    fn eval(&self, context: &Context) -> Value;
}


struct ValueNode {
    pub value: Value,
}
impl AstNode for ValueNode {
    fn eval(&self, context: &Context) -> Value {
        context.get(&self.value).unwrap_or(&self.value).clone()
    }
}


named!(value<&[u8], ValueNode>, chain!(
    val: map_res!(alt!(tag!("_") | alphanumeric), str::from_utf8),
    || { ValueNode{value: val.to_string()} }
));


pub fn parse(input: &str) -> Option<Box<AstNode>> {
    match value(input.as_bytes()) {
        IResult::Done(_, node) => Some(Box::new(node)),
        _ => None,
    }
}
