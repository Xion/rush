//! Value type.
//!
//! Everything that expressions can operate on is encapsulated into values,
//! which are tagged unions (Rust enums) with the basic types as variants.
//!
//! This module implements the Value type itself, as well as all the various
//! conversions to and from Rust types, and serialization formats like JSON.

mod cmp;
mod types;


use std::convert::From;
use std::fmt;
use std::str::FromStr;

use conv::TryFrom;
use conv::errors::{GeneralError, NoError};
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

// Conversions from Rust types

/// Macro to create a straighforward From<FooRepr> -> Value::Foo implementation.
macro_rules! value_from (
    ($input:ty => $output:ident) => {
        impl From<$input> for Value {
            #[inline(always)]
            fn from(input: $input) -> Self {
                Value::$output(input)
            }
        }
    }
);

// Note how string input is deliberately omitted, for it is ambiguous.
// (It could result in either Value::String or Value::Symbol).
value_from!(BooleanRepr => Boolean);
value_from!(IntegerRepr => Integer);
value_from!(FloatRepr => Float);
value_from!(RegexRepr => Regex);
value_from!(ArrayRepr => Array);
value_from!(ObjectRepr => Object);
value_from!(FunctionRepr => Function);


impl From<char> for Value {
    fn from(input: char) -> Self {
        let mut string = String::new();
        string.push(input);
        Value::String(string)
    }
}

impl From<u8> for Value {
    fn from(input: u8) -> Self {
        Value::Integer(input as IntegerRepr)
    }
}


// This is a somewhat special "default" and "intelligent" conversion that's
// automatically applied to the expresion's input (the _ symbol).
impl FromStr for Value {
    type Err = NoError;

    /// Create a Value from string, reinterpreting input as number
    /// if we find out it's in numeric form.
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(int) = s.parse::<IntegerRepr>() {
            return Ok(Value::Integer(int));
        }
        if let Ok(float) = s.parse::<FloatRepr>() {
            return Ok(Value::Float(float));
        }
        if let Ok(boolean) = s.parse::<BooleanRepr>() {
            return Ok(Value::Boolean(boolean));
        }
        Ok(Value::String(s.to_owned()))
    }
}


// Producing string output

impl TryFrom<Value> for String {
    type Err = GeneralError<&'static str>;

    #[inline(always)]
    fn try_from(src: Value) -> Result<Self, Self::Err> {
        String::try_from(&src)
    }
}
impl<'a> TryFrom<&'a Value> for String {
    type Err = <String as TryFrom<Value>>::Err;

    /// Try to convert a Value to string that can be emitted
    /// as a final result of a computation.
    fn try_from(src: &'a Value) -> Result<Self, Self::Err> {
        match *src {
            Value::Empty => Err(GeneralError::Unrepresentable(
                "cannot serialize an empty value"
            )),
            Value::Symbol(ref t) => Ok(format!("{}", t)),
            Value::Boolean(ref b) => Ok(format!("{}", b)),
            Value::Integer(ref i) => Ok(format!("{}", i)),
            Value::Float(ref f) => {
                // always include decimal point and zero, even if the float
                // is actually an integer
                let mut res = f.to_string();
                if !res.contains(".") {
                    res.push_str(".0");
                }
                Ok(format!("{}", res))
            },
            Value::String(ref s) => Ok(s.clone()),
            Value::Regex(..) => Err(GeneralError::Unrepresentable(
                "cannot serialize a regex"
            )),
            Value::Array(ref a) => {
                // for final display, an array is assumed to contain lines of output
                Ok(format!("{}", a.iter()
                    .map(|v| format!("{}", v)).collect::<Vec<String>>()
                    .join("\n")))
            },
            Value::Object(..) => Ok(src.to_json().to_string()),
            Value::Function(..) => Err(GeneralError::Unrepresentable(
                "cannot serialize a function"
            )),
        }
    }
}

impl fmt::Display for Value {
    /// Format a Value for outputing it as a result of the computation.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        String::try_from(self)
            .map(|s| write!(fmt, "{}", s))
            // TODO(xion): return an Err(fmt::Error) rather than panicking
            // when formatting constructs actually react to it constructively
            .expect(&format!("can't display a value of type `{}`", self.typename()))
    }
}


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
