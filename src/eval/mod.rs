//! Module implementing evaluation of parsed expressions.

#[macro_use]
mod util;

mod context;
mod functions;
mod value;

pub use self::context::Context;
pub use self::value::Value;


use std::error;
use std::fmt;
use std::result;


/// Error that may have occurred during evaluation.
#[derive(Clone,Debug)]
pub struct Error {
    // TODO(xion): include the representation of the eval'd AST node
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error{message: msg.to_owned()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eval error: {}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { &self.message }
    fn cause(&self) -> Option<&error::Error> { None }
}


/// Result of an evaluation attempt.
pub type Result = result::Result<Value, Error>;


/// Trait for objects that can be evaluated within given Context.
pub trait Eval : fmt::Debug {
    fn eval(&self, context: &Context) -> Result;
}
