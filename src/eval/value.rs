//! Value type.

use std::fmt;
use std::str::FromStr;


/// Typed value that's operated upon.
#[derive(Clone,PartialEq)]
pub enum Value {
    /// No value at all.
    Empty,

    /// Symbol is a string that can be interpreted as a variable name.
    ///
    /// `Symbol("x")` shall evaluate to the value of variable `x` if one is in scope.
    /// Otherwise, it should be equivalent to String("x").
    Symbol(String),

    // Various data types.
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    // TODO(xion): function type
}


impl Value {
    pub fn map_str<F: FnOnce(&str) -> String>(&self, func: F) -> Option<Value> {
        if let &Value::String(ref s) = self {
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
    pub fn map_bool<F: FnOnce(bool) -> bool>(&self, func: F) -> Option<Value> {
        if let Value::Boolean(b) = *self {
            return Some(Value::Boolean(func(b)));
        }
        None
    }

    pub fn to_string_value(&self) -> Option<Value> {
        match *self {
            Value::String(_) => Some(self.clone()),
            Value::Integer(i) => Some(Value::String(i.to_string())),
            Value::Float(f) => Some(Value::String(f.to_string())),
            Value::Boolean(b) => Some(Value::String((
                if b { "true" } else { "false" }
            ).to_string())),
            _ => None,
        }
    }
    pub fn to_int_value(&self) -> Option<Value> {
        match *self {
            Value::String(ref s) => s.parse::<i64>().ok().map(Value::Integer),
            Value::Integer(_) => Some(self.clone()),
            Value::Float(f) => Some(Value::Integer(f as i64)),
            Value::Boolean(b) => Some(Value::Integer(if b { 1 } else { 0 })),
            _ => None,
        }
    }
    pub fn to_float_value(&self) -> Option<Value> {
        match *self {
            Value::String(ref s) => s.parse::<f64>().ok().map(Value::Float),
            Value::Integer(i) => Some(Value::Float(i as f64)),
            Value::Float(_) => Some(self.clone()),
            Value::Boolean(b) => Some(Value::Float(if b { 1.0 } else { 0.0 })),
            _ => None,
        }
    }
}


impl FromStr for Value {
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
        if let Ok(boolean) = s.parse::<bool>() {
            return Ok(Value::Boolean(boolean));
        }
        Ok(Value::String(s.to_string()))
    }
}


impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, ":{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b.to_string()),
            Value::Integer(ref i) => write!(fmt, "{}i", i),
            Value::Float(ref f) => write!(fmt, "{}f", f),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, "{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b),
            Value::Integer(ref i) => write!(fmt, "{}", i),
            Value::Float(ref f) => {
                // always include decimal point and zero, even if the float
                // is actually an integer
                let mut res = f.to_string();
                if !res.contains(".") {
                    res.push_str(".0");
                }
                write!(fmt, "{}", res)
            },
            Value::String(ref s) => write!(fmt, "{}", s),
        }
    }
}
