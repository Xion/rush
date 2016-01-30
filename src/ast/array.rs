//! Module implementing the AST node that represents an expression for creating an array.

use std::fmt;

use eval::Eval;


pub struct ArrayNode {
    pub elements: Vec<Box<Eval>>
}

impl fmt::Debug for ArrayNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Array: [{}]>", self.elements.iter()
            .map(|ref elem| format!("{:?}", elem))
            .collect::<Vec<String>>().join(","))
    }
}
