//! Module implementing an AST node for binary operations.

use std::fmt;

use eval::Eval;


/// Represents an operation involving binary operators and their arguments.
///
/// Because of the way the operations are parsed, arbitrary length chains
/// of operations with the same priority (e.g. + and -) are represented
/// as one object.
///
pub struct BinaryOpNode {
    pub first: Box<Eval>,
    pub rest: Vec<(String, Box<Eval>)>,
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Op: {:?} {}>", self.first,
               self.rest.iter()
                   .map(|&(ref op, ref arg)| format!("{} {:?}", op, arg))
                   .collect::<Vec<String>>().join(" "))
    }
}

