//! Module implementing evaluation of parsed expressions.

use std::collections::HashMap;
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

impl FromStr for Value {
    // TODO(xion): better error type
    type Err = ();

    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(float) = s.parse::<f64>() {
            return Ok(Value::Float(float));
        }
        if let Ok(int) = s.parse::<i64>() {
            return Ok(Value::Integer(int));
        }
        // TODO(xion): strip quotes
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
        let mut funcs = Functions::new();
        funcs.insert("abs".to_string(), Box::new(|args: Vec<Value>| {
            if let Value::Float(ref f) = args[0] {
                return Value::Float(f.abs());
            }
            panic!("invalid arguments to abs()");
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
#[derive(Clone)]
pub struct Error {
    pub message: String,
}
impl Error {
    pub fn err<T>(msg: &str) -> Result<T, Error> {
        Err(Error{message: msg.to_string()})
    }
}

/// Trait for objects that can be evaluated within given Context.
pub trait Eval {
    fn eval(&self, context: &Context) -> Result<Value, Error>;
}
