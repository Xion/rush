//! Value type.

use std::fmt;
use std::str::FromStr;

use super::context::Context;


/// Typed value that's operated upon.
#[derive(Clone,Debug,PartialEq)]
pub enum Value {
    /// No value at all.
    Empty,

    /// Reference to a variable of given name.
    ///
    /// If the variable is not found in the scope, the name is interpreted
    /// verbatim as a String.
    Reference(String),

    // Various data types.
    String(String),
    Integer(i64),
    Float(f64),
    // TODO(xion): function type
}

impl Value {
    /// Resolve a possible variable reference against given context.
    ///
    /// Returns the variable's Value (which may be just variable name as string),
    /// or a copy of the original Value if it wasn't a reference.
    pub fn resolve(&self, context: &Context) -> Value {
        match *self {
            Value::Reference(ref t) => context.get_var(t)
                .map(|v| v.clone())
                .unwrap_or_else(|| Value::String(t.clone())),
            _ => self.clone(),
        }
    }

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
        if let Ok(float) = s.parse::<f64>() {
            return Ok(Value::Float(float));
        }
        if let Ok(int) = s.parse::<i64>() {
            return Ok(Value::Integer(int));
        }

        // quoted string literals are always interpreted as strings,
        // whereas unquoted identifiers may be variable references
        let mut s = s.to_string();
        if s.is_empty() {
            Ok(Value::String(s))
        } else if s.starts_with("\"") && s.ends_with("\"") {
            s.pop().unwrap();
            s.remove(0);
            Ok(Value::String(s))
        } else {
            Ok(Value::Reference(s))
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
            Value::Integer(ref i) => write!(fmt, "{}", i),
            Value::Float(ref f) => write!(fmt, "{}", f),
            Value::Reference(ref t) => write!(fmt, "{}", t),
            // _ => write!(fmt, "{}", "<unknown>")
        }
    }
}
