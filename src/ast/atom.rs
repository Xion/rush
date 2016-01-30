//! Module implementing the AST node that represents an expression "atom".

use std::fmt;
use std::str::FromStr;

use eval::Value;


/// Represents the smallest, indivisible unit of an expression: a single value.
pub struct AtomNode {
    pub value: Value,
}

impl fmt::Debug for AtomNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Atom: {:?}>", self.value)
    }
}

impl FromStr for AtomNode {
    type Err = <Value as FromStr>::Err;

    fn from_str(s: &str) -> Result<AtomNode, Self::Err> {
        s.parse::<Value>().map(|v| AtomNode{value: v})
    }
}
