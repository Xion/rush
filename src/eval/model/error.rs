//! Evaluation error.

use std::error::Error as StdError;
use std::fmt;


/// Error that may have occurred during evaluation.
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Error {
    // TODO(xion): include the representation of the eval'd AST node
    pub message: String,
}

impl Error {
    #[inline(always)]
    pub fn new(msg: &str) -> Error {
        Error{message: msg.to_owned()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eval error: {}", self.message)
    }
}

impl StdError for Error {
    #[inline(always)] fn description(&self) -> &str { &self.message }
    #[inline(always)] fn cause(&self) -> Option<&StdError> { None }
}
