//! Comparison and ordering of Value types.

use std::cmp::{Ordering, PartialOrd};

use eval;
use eval::util::cmp::{TryEq, TryOrd};
use super::Value;
use super::types::FloatRepr;


impl TryOrd for Value {
    type Err = eval::Error;

    fn try_cmp(&self, other: &Value) -> Result<Ordering, Self::Err> {
        match (self, other) {
            (&Value::Integer(a), &Value::Integer(b)) => a.partial_cmp(&b),
            (&Value::Integer(a), &Value::Float(b)) => (a as FloatRepr).partial_cmp(&b),
            (&Value::Float(a), &Value::Integer(b)) => a.partial_cmp(&(b as FloatRepr)),
            (&Value::Float(a), &Value::Float(b)) => a.partial_cmp(&b),

            (&Value::String(ref a), &Value::String(ref b)) => a.partial_cmp(b),
            // TODO(xion): consider implementing ordering of arrays, too

            _ => None,
        }.ok_or_else(|| eval::Error::new(&format!(
            "cannot compare {} with {}", self.typename(), other.typename()
        )))
    }
}
impl_partialord_for_tryord!(Value);


impl TryEq for Value {
    type Err = eval::Error;

    fn try_eq(&self, other: &Value) -> Result<bool, Self::Err> {
        match (self, other) {
            // numeric types
            (&Value::Integer(a), &Value::Integer(b)) => Ok(a == b),
            (&Value::Integer(a), &Value::Float(b)) => Ok((a as FloatRepr) == b),
            (&Value::Float(a), &Value::Integer(b)) => Ok(a == (b as FloatRepr)),
            (&Value::Float(a), &Value::Float(b)) => Ok(a == b),

            // others
            (&Value::Boolean(a), &Value::Boolean(b)) => Ok(a == b),
            (&Value::String(ref a), &Value::String(ref b)) => Ok(a == b),
            (&Value::Array(ref a), &Value::Array(ref b)) => Ok(a == b),
            (&Value::Object(ref a), &Value::Object(ref b)) => Ok(a == b),

            _ => Err(eval::Error::new(&format!(
                "cannot compare {} with {}", self.typename(), other.typename()
            ))),
        }
    }
}
impl_partialeq_for_tryeq!(Value);
