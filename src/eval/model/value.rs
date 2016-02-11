//! Value type.

use std::collections::HashMap;
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
    Object(HashMap<String, Value>),
    // TODO(xion): function type
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
            Value::Array(..) => "array",
            Value::Object(..) => "object",
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
    pub fn unwrap_object(self) -> HashMap<String, Value> {
        match self {
            Value::Object(o) => o,
            _ => { panic!("unwrap_object() on {} value", self.typename()) },
        }
    }
}


// TODO(xion): given the numerous ways we can & want to interpret the input,
// it makes less and less sense to has this as default;
// consider removing this impl
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
            // TODO(xion): this will probably be _almost_ like JSON
            // except that values should be formatted as {:?}, of course
            Value::Object(ref o) => unimplemented!(),
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
            // TODO(xion): object should serialize as JSON
            Value::Object(ref o) => unimplemented!(),
        }
    }
}
