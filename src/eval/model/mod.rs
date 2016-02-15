//! Module defining the data structures that hold the necessary state
//! that's used while evaluating expressions.

mod context;
mod function;
mod functions;
mod variables;

pub mod value;

pub use self::context::Context;
pub use self::value::Value;
