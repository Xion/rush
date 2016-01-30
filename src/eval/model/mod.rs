//! Module defining the data structures that hold the necessary state
//! while evaluating expressions.

mod context;
mod functions;
mod value;
mod variables;

pub use self::context::Context;
pub use self::value::Value;
