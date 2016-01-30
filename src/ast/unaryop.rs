//! Module implementing the AST node for unary operation.

use std::fmt;

use eval::Eval;


/// Represents an operation involving a unary operator and its argument.
pub struct UnaryOpNode {
    pub op: String,
    pub arg: Box<Eval>,
}

impl fmt::Debug for UnaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Op: {}{:?}>", self.op, self.arg)
    }
}
