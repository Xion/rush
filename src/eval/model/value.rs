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
    Array(Vec<Value>),
    // TODO(xion): function type
}


impl Value {
    /// Return the type of this value as string.
    /// These names are user-facing, e.g. they can occur inside error messages.
    pub fn typename(&self) -> &str {
        match *self {
            Value::Empty => "empty",
            Value::Symbol(..) => "symbol",
            Value::Boolean(..) => "bool",
            Value::Integer(..) => "int",
            Value::Float(..) => "float",
            Value::String(..) => "str",
            Value::Array(..) => "array",
        }
    }

    // TODO(xion): roll those methods into str, int, float, bool builtins
    // which are their only users
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
    pub fn to_bool_value(&self) -> Option<Value> {
        match *self {
            Value::String(ref s) => s.parse::<bool>().ok().map(Value::Boolean),
            Value::Integer(i) => Some(Value::Boolean(i != 0)),
            Value::Float(f) => Some(Value::Boolean(f != 0.0)),
            Value::Boolean(_) => Some(self.clone()),
            Value::Array(ref a) => Some(Value::Boolean(a.len() > 0)),
            _ => None,
        }
    }
    pub fn to_array_value(&self) -> Option<Value> {
        match *self {
            Value::Array(_) => Some(self.clone()),
            _ => None,
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Value::String(s) => s,
            _ => { panic!("unwrap_string() on {} value", self.typename()) },
        }
    }
    pub fn unwrap_int(self) -> i64 {
        match self {
            Value::Integer(i) => i,
            _ => { panic!("unwrap_int() on {} value", self.typename()) },
        }
    }
    pub fn unwrap_float(self) -> f64 {
        match self {
            Value::Float(f) => f,
            _ => { panic!("unwrap_float() on {} value", self.typename()) },
        }
    }
    pub fn unwrap_bool(self) -> bool {
        match self {
            Value::Boolean(b) => b,
            _ => { panic!("unwrap_bool() on {} value", self.typename()) },
        }
    }
    pub fn unwrap_array(self) -> Vec<Value> {
        match self {
            Value::Array(a) => a,
            _ => { panic!("unwrap_array() on {} value", self.typename()) },
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
    /// Format a Value for debugging purposes.
    /// This representation is not meant for consumption by end users.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, ":{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b.to_string()),
            Value::Integer(ref i) => write!(fmt, "{}i", i),
            Value::Float(ref f) => write!(fmt, "{}f", f),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
            Value::Array(ref a) => {
                write!(fmt, "[{}]", a.iter()
                    .map(|v| format!("{:?}", v)).collect::<Vec<String>>()
                    .join(","))
            },
        }
    }
}


impl fmt::Display for Value {
    /// Format a Value for outputing it as a result of the computation.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TODO(xion): make Empty a formatting error
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
            Value::Array(ref a) => {
                // for final display, an array is assummed to contain lines of output
                write!(fmt, "{}", a.iter()
                    .map(|v| format!("{}", v)).collect::<Vec<String>>()
                    .join("\n"))
            },
        }
    }
}
