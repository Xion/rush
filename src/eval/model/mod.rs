//! Module defining the data structures that hold the necessary state
//! that's used while evaluating expressions.

mod context;
pub mod function;
pub mod value;

pub use self::context::Context;
pub use self::function::{Args, Function, Invoke};
pub use self::value::Value;
