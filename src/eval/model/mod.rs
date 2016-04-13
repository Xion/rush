//! Module defining the data structures that hold the necessary state
//! that's used while evaluating expressions.

mod arity;
mod context;
pub mod function;
pub mod value;

pub use self::arity::{Args, ArgCount, Arity};
pub use self::context::{Context, Name};
pub use self::function::{Function, Invoke};
pub use self::value::Value;
