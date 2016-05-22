//! Value type.
//!
//! Everything that expressions can operate on is encapsulated into values,
//! which are tagged unions (Rust enums) with the basic types as variants.
//!
//! This module implements the Value type itself, as well as all the various
//! conversions to and from Rust types, and serialization formats like JSON.

mod cmp;
mod conv;
mod output;
mod types;


use std::convert::From;
use std::fmt;

use conv::misc::InvalidSentinel;
use rustc_serialize::json::{Json, ToJson};

pub use self::types::*;


/// Typed value that's operated upon.
#[derive(Clone)]
pub enum Value {
    /// No value at all.
    Empty,

    /// Symbol is a string that can be interpreted as a variable name.
    ///
    /// `Symbol("x")` shall evaluate to the value of variable `x` if one is in scope.
    /// Otherwise, it is an equivalent to String("x").
    Symbol(SymbolRepr),

    // Various data types.
    Boolean(BooleanRepr),
    Integer(IntegerRepr),
    Float(FloatRepr),
    String(StringRepr),
    Regex(RegexRepr),
    Array(ArrayRepr),
    Object(ObjectRepr),
    Function(FunctionRepr),
}

impl Value {
    /// Return the type of this value as string.
    /// These names are user-facing, e.g. they can occur inside error messages.
    pub fn typename(&self) -> &'static str {
        match *self {
            Value::Empty => "empty",
            Value::Symbol(..) => "symbol",
            Value::Boolean(..) => "bool",
            Value::Integer(..) => "int",
            Value::Float(..) => "float",
            Value::String(..) => "str",
            Value::Regex(..) => "regex",
            Value::Array(..) => "array",
            Value::Object(..) => "object",
            Value::Function(..) => "function",
        }
    }
}

impl InvalidSentinel for Value {
    #[inline(always)]
    fn invalid_sentinel() -> Self { Value::Empty }
}

impl fmt::Debug for Value {
    /// Format a Value for debugging purposes.
    /// This representation is not meant for consumption by end users.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, "'{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b.to_string()),
            Value::Integer(ref i) => write!(fmt, "{}i", i),
            Value::Float(ref f) => write!(fmt, "{}f", f),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
            Value::Regex(ref r) => write!(fmt, "/{}/", r.as_str()),
            Value::Array(ref a) => {
                write!(fmt, "[{}]", a.iter()
                    .map(|v| format!("{:?}", v)).collect::<Vec<String>>()
                    .join(","))
            },
            Value::Object(ref o) => {
                write!(fmt, "{{{}}}", o.iter()
                    .map(|(k, v)| format!("\"{}\": {:?}", k, v))
                    .collect::<Vec<String>>().join(","))
            },
            Value::Function(ref f) => write!(fmt, "{:?}", f),
        }
    }
}


// TODO(xion): extract the impl's below to submodules


// JSON conversions

impl From<Json> for Value {
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
    /// This is used for printing Object values as final output.
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
