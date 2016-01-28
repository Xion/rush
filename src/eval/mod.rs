//! Module implementing evaluation of parsed expressions.

mod context;
mod functions;
mod value;

pub use self::context::Context;
pub use self::value::Value;


use std::error;
use std::fmt;


/// Error that may have occurred during evaluation.
#[derive(Clone,Debug)]
pub struct Error {
    // TODO(xion): include the representation of the eval'd AST node
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error{message: msg.to_string()}
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
// TODO(xion): rename it to just Result, so it'll be used as eval::Result


/// Trait for objects that can be evaluated within given Context.
pub trait Eval : fmt::Debug {
    fn eval(&self, context: &Context) -> EvalResult;
}
