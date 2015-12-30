//! Module implementing evaluation of parsed expressions.

use std::collections::HashMap;
use std::error;
use std::fmt::{self, Display};
use std::str::FromStr;


/// Typed value that's operated upon.
#[derive(Clone,Debug,PartialEq)]
pub enum Value {
    Empty,
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

    fn map_str<F: FnOnce(&str) -> String>(&self, func: F) -> Option<Value> {
        if let Value::String(ref s) = *self {
            return Some(Value::String(func(s)));
        }
        None
    }

    fn map_string<F: FnOnce(String) -> String>(self, func: F) -> Option<Value> {
        if let Value::String(s) = self {
            return Some(Value::String(func(s)));
        }
        None
    }

    fn map_int<F: FnOnce(i64) -> i64>(&self, func: F) -> Option<Value> {
        if let Value::Integer(i) = *self {
            return Some(Value::Integer(func(i)));
        }
        None
    }

    fn map_float<F: FnOnce(f64) -> f64>(&self, func: F) -> Option<Value> {
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
        Ok(Value::String(s.to_string()))
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::String(ref s) => write!(fmt, "{}", s),
            Value::Integer(ref i) => write!(fmt, "{}", i),
            Value::Float(ref f) => write!(fmt, "{}", f),
            // _ => write!(fmt, "{}", "<unknown>")
        }
    }
}


type Functions = HashMap<String, Box<Fn(Vec<Value>) -> Value>>;
type Variables = HashMap<String, Value>;

/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluation an expression.
pub struct Context {
    vars: Variables,
    funcs: Functions,
}

impl Context {
    pub fn new() -> Context {
        // TODO(xion): consider making Functions a struct and extracting
        // the boilerplate for defining functions there
        let mut funcs = Functions::new();
        funcs.insert("abs".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_float(f64::abs).expect("invalid arguments to abs()")
        }));
        funcs.insert("abs".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_int(i64::abs).expect("invalid arguments to abs()")
        }));
        funcs.insert("rev".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_str(|s: &str| {
                // TODO(xion): since this reverses chars not graphemes,
                // it mangles some non-Latin strings;
                // fix with unicode-segmentation crate
                s.chars().rev().collect::<String>()
            }).expect("invalid arguments to rev()")
        }));

        Context{vars: Variables::new(), funcs: funcs}
    }

    /// Retrieves the value for a variable if it exists.
    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.vars.get(&name.to_string())
    }

    /// Set a value for a variable.
    /// If the variable didn't exist before, it is created.
    pub fn set_var(&mut self, name: &str, value: Value) {
        let name = name.to_string();
        if let Some(val) = self.vars.get_mut(&name) {
            *val = value;
            return
        }
        self.vars.insert(name, value);
    }

    /// Set a string value for a variable.
    pub fn set_string_var(&mut self, name: &str, value: &str) {
        self.set_var(name, Value::String(value.to_string()))
    }

    /// Call a function of given name with given arguments.
    /// Returns result or None if the function couldn't be found.
    pub fn call_func(&self, name: &str, args: Vec<Value>) -> Option<Value> {
        self.funcs.get(&name.to_string()).map(|func| func(args))
    }
}


/// Error that may have occurred during evaluation.
#[derive(Clone,Debug)]
pub struct Error {
    pub message: String,
}
impl Error {
    pub fn err<T>(msg: &str) -> Result<T, Error> {
        Err(Error{message: msg.to_string()})
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Eval error: {}", self.message)
    }
}
impl error::Error for Error {
    fn description(&self) -> &str { &self.message }
    fn cause(&self) -> Option<&error::Error> { None }
}

/// Result of an evaluation attempt.
pub type EvalResult = Result<Value, Error>;

/// Trait for objects that can be evaluated within given Context.
pub trait Eval {
    fn eval(&self, context: &Context) -> EvalResult;
}
