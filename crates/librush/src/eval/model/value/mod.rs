//! Value type.
//!
//! Everything that expressions can operate on is encapsulated into values,
//! which are tagged unions (Rust enums) with the basic types as variants.
//!
//! This module implements the Value type itself, as well as all the various
//! conversions to and from Rust types, and serialization formats like JSON.

mod cmp;
mod conv;
mod json;
mod output;
mod types;


use std::fmt;
use std::num::FpCategory;

use conv::misc::InvalidSentinel;

pub use self::types::*;


// TODO: make Value an alias like: type Value = Rc<Data>;
// and Data should contain actual, well, data.
// this is because currently, we are cloning the values way too much
// and we won't really be able to fix that


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
            Value::String(..) => "string",
            Value::Regex(..) => "regex",
            Value::Array(..) => "array",
            Value::Object(..) => "object",
            Value::Function(..) => "function",
        }
    }
}

impl InvalidSentinel for Value {
    #[inline]
    fn invalid_sentinel() -> Self { Value::Empty }
}

impl fmt::Debug for Value {
    /// Format a Value for debugging purposes.
    /// This representation is not meant for consumption by end users.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "nil"),
            Value::Symbol(ref t) => write!(fmt, "'{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b.to_string()),
            Value::Integer(ref i) => write!(fmt, "{}i", i),
            Value::Float(ref f) => {
                match f.classify() {
                    FpCategory::Nan => write!(fmt, "NaN"),
                    FpCategory::Infinite => write!(fmt, "Inf"),
                    _ => write!(fmt, "{}f", f),
                }
            },
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
