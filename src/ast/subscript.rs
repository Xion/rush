//! Module implementing the AST node that represents an array subscript operation,
//! i.e. an expression in the form of `x[i]`.

use std::fmt;

use eval::Eval;


/// Represents an operation of taking a subscript of an object ("indexing").
/// The object is commonly an array or a string.
pub struct SubscriptNode {
    pub object: Box<Eval>,
    pub index: Box<Eval>,
}

impl fmt::Debug for SubscriptNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Index: {:?}[{:?}]>", self.object, self.index)
    }
}
