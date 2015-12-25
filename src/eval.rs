//! Module implementing evaluation of parsed expressions.

use std::collections::hash_map::HashMap;


// TODO(xion): make an ADT or something, to represent various types of values
pub type Value = String;
pub type Context = HashMap<String, Value>;

pub trait Eval {
    fn eval(&self, context: &Context) -> Value;
}
