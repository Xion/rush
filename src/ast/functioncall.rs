//! Module implementing AST node that represents a function call.

use std::fmt;

use eval::Eval;


/// Represents a call to a function with given name and arguments.
///
/// The exact function the name resolves to depends on the context passed
/// during evaluation.
pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}

impl fmt::Debug for FunctionCallNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Call: {}({})>", self.name,
               self.args.iter()
                   .map(|ref arg| format!("{:?}", arg))
                   .collect::<Vec<String>>().join(","))
    }
}

