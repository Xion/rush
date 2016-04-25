//! Module implementing evaluation of parsed expressions.
#![allow(dead_code)]

#[macro_use]
pub mod util;
pub mod model;

mod api;
mod atoms;
mod blocks;
mod operators;
mod trailers;

pub use self::model::{Context, Function, Invoke, Value};
pub use self::model::value;  // for *Repr typedefs


use std::error::Error as StdError;
use std::fmt;
use std::result;

use mopa;


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


/// Result of an evaluation attempt.
pub type Result = result::Result<Value, Error>;


/// Trait for objects that can be evaluated within given Context.
pub trait Eval : fmt::Debug + mopa::Any {
    fn eval(&self, context: &mut Context) -> Result;
}
mopafy!(Eval);
