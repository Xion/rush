//! JSON conversions for Value.

use std::convert::From;

use conv::TryFrom;
use conv::errors::GeneralError;
use rustc_serialize::json::{Json, ToJson};

use super::types::IntegerRepr;
use super::Value;


impl TryFrom<Json> for Value {
    type Err = GeneralError<String>;

    /// Parse a JSON object into a value.
    fn try_from(src: Json) -> Result<Self, Self::Err> {
        match src {
            Json::Null => Ok(Value::Empty),
            Json::Boolean(b) => Ok(Value::Boolean(b)),
            Json::I64(i) => Ok(Value::Integer(i)),
            Json::U64(u) => {
                if u > (IntegerRepr::max_value() as u64) {
                    return Err(GeneralError::PosOverflow(
                        format!("JSON integer too large: {}", u)
                    ));
                }
                Ok(Value::Integer(u as IntegerRepr))
            },
            Json::F64(f) => Ok(Value::Float(f)),
            Json::String(s) => Ok(Value::String(s)),
            Json::Array(a) => Ok(Value::Array(
                a.into_iter().map(Value::from).collect()
            )),
            Json::Object(o) => Ok(Value::Object(
                o.into_iter().map(|(k, v)| (k, Value::from(v))).collect()
            )),
        }
    }
}

impl From<Json> for Value {
    fn from(input: Json) -> Self {
        Value::try_from(input).unwrap()
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
