//! Value type.

use std::fmt;
use std::str::FromStr;


/// Typed value that's operated upon.
#[derive(Clone,Debug,PartialEq)]
pub enum Value {
    /// No value at all.
    Empty,

    /// Symbol is a string that can be interpreted as a variable name.
    ///
    /// `Symbol("x")` shall evaluate to the value of variable `x` if one is in scope.
    /// Otherwise, it should be equivalent to String("x").
    Symbol(String),

    // Various data types.
    String(String),
    Integer(i64),
    Float(f64),
    // TODO(xion): function type
}

impl Value {
    pub fn as_string(self) -> Option<String> {
        return match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn map_str<F: FnOnce(&str) -> String>(&self, func: F) -> Option<Value> {
        if let Value::String(ref s) = *self {
            return Some(Value::String(func(s)));
        }
        None
    }

    pub fn map_string<F: FnOnce(String) -> String>(self, func: F) -> Option<Value> {
        if let Value::String(s) = self {
            return Some(Value::String(func(s)));
        }
        None
    }

    pub fn map_int<F: FnOnce(i64) -> i64>(&self, func: F) -> Option<Value> {
        if let Value::Integer(i) = *self {
            return Some(Value::Integer(func(i)));
        }
        None
    }

    pub fn map_float<F: FnOnce(f64) -> f64>(&self, func: F) -> Option<Value> {
        if let Value::Float(f) = *self {
            return Some(Value::Float(func(f)))
        }
        None
    }
}

impl FromStr for Value {
    // TODO(xion): better error type
    type Err = ();

    /// Create a Value from string, reinterpreting input as number
    /// if we find out it's in numeric form.
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(int) = s.parse::<i64>() {
            return Ok(Value::Integer(int));
        }
        if let Ok(float) = s.parse::<f64>() {
            return Ok(Value::Float(float));
        }

        // quoted string literals are always interpreted as strings,
        // whereas unquoted identifiers are symbols and may be variable references
        let mut s = s.to_string();
        if s.is_empty() {
            Ok(Value::String(s))
        } else if s.starts_with("\"") && s.ends_with("\"") {
            s.pop().unwrap();
            s.remove(0);
            Ok(Value::String(s))
        } else {
            // TODO(xion): check if content is alphanumeric
            Ok(Value::Symbol(s))
        }
    }
}

// TODO(xion): implement custom Debug for more user-friendly error messages
impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, "{}", t),
            Value::String(ref s) => write!(fmt, "{}", s),
            Value::Integer(ref i) => write!(fmt, "{}", i),
            Value::Float(ref f) => write!(fmt, "{}", f),
            // _ => write!(fmt, "{}", "<unknown>")
        }
    }
}
