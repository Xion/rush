//! Module implementing evaluation of parsed expressions.
//
//! Currently, the evaluator is a recursive descent over the AST.

#![allow(doc_markdown)]
#![allow(needless_borrow)]
#![allow(option_map_unwrap_or)]
#![allow(ptr_arg)]
#![allow(transmute_ptr_to_ref)]

#[macro_use]
pub mod util;
#[macro_use]
pub mod model;

mod api;
mod atoms;
mod operators;
mod trailers;

pub use self::model::{Context, Function, Invoke, Value};
pub use self::model::Error;
pub use self::model::value;  // for *Repr typedefs


use std::fmt;
use std::result;

use mopa;


/// Result of an evaluation attempt.
pub type Result = result::Result<Value, Error>;


/// Trait for objects that can be evaluated within given Context.
pub trait Eval : fmt::Debug + mopa::Any + 'static {
    fn eval(&self, context: &mut Context) -> Result;
}
mopafy!(Eval);
