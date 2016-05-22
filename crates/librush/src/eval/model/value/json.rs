//! JSON conversions for Value.

use std::convert::From;

use rustc_serialize::json::{Json, ToJson};

use super::types::IntegerRepr;
use super::Value;


impl From<Json> for Value {
    /// Parse a JSON object into a value.
    fn from(input: Json) -> Self {
        match input {
            Json::Null => Value::Empty,
            Json::Boolean(b) => Value::Boolean(b),
            Json::I64(i) => Value::Integer(i),
            Json::U64(u) => {
                // TODO(xion): implement optional parsing using TryFrom
                if u > (IntegerRepr::max_value() as u64) {
                    panic!("JSON integer too large: {}", u);
                }
                Value::Integer(u as IntegerRepr)
            },
            Json::F64(f) => Value::Float(f),
            Json::String(s) => Value::String(s),
            Json::Array(a) => Value::Array(
                a.into_iter().map(Value::from).collect()
            ),
            Json::Object(o) => Value::Object(
                o.into_iter().map(|(k, v)| (k, Value::from(v))).collect()
            ),
        }
    }
}


impl ToJson for Value {
    /// Format the value as JSON.
    ///
    /// This is used for a variety of things, including the json() function
    /// and printing of Object values as final output.
    fn to_json(&self) -> Json {
        match *self {
            Value::Empty => Json::Null,
            Value::Symbol(ref t) => Json::String(t.clone()),
            Value::Boolean(b) => Json::Boolean(b),
            Value::Integer(i) => Json::I64(i),
            Value::Float(f) => Json::F64(f),
            Value::String(ref s) => Json::String(s.clone()),
            Value::Regex(ref r) => Json::String(r.as_str().to_owned()),
            Value::Array(ref a) => Json::Array(
                a.iter().map(|v| v.to_json()).collect()
            ),
            Value::Object(ref o) => Json::Object(
                o.iter().map(|(k, v)| (k.clone(), v.to_json())).collect()
            ),
            Value::Function(..) => panic!("function cannot be serialized as JSON"),
        }
    }
}
