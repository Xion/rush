//! Data structures representing the abstract syntax tree (AST)
//! of parsed expressions.
//!
//! For the code that evaluates those nodes, see the `eval` module.

use std::fmt;
use std::str::FromStr;

use eval::{Eval, Value};


/// AST node representing the smallest, indivisible unit of an expression:
/// a single scalar value.
pub struct ScalarNode {
    pub value: Value,
}

impl fmt::Debug for ScalarNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Atom: {:?}>", self.value)
    }
}

// TODO(xion): note from `impl FromStr for Value` applies here, too
impl FromStr for ScalarNode {
    type Err = <Value as FromStr>::Err;

    fn from_str(s: &str) -> Result<ScalarNode, Self::Err> {
        s.parse::<Value>().map(|v| ScalarNode{value: v})
    }
}


/// AST node representing the expression for creating a new array of values.
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


/// AST node repreenting an operation involving a unary operator and its argument.
pub struct UnaryOpNode {
    pub op: String,
    pub arg: Box<Eval>,
}

impl fmt::Debug for UnaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Op: {}{:?}>", self.op, self.arg)
    }
}


/// AST node representing an operation involving binary operators
/// and their arguments.
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


/// AST node representing an operation of taking a subscript of an object
/// (also referred to as "indexing").
///
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


/// AST node representing a call to a function with given name and arguments.
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


/// AST node representing a conditional choice based on a boolean value.
///
/// Syntactically, this could be a ternary operator (foo ? x : y)
/// or even a full-blown `if` statement.
pub struct ConditionalNode {
    pub cond: Box<Eval>,
    pub then: Box<Eval>,
    pub else_: Box<Eval>,
}

impl fmt::Debug for ConditionalNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<If: {:?} then {:?} else {:?}>",
               self.cond, self.then, self.else_)
    }
}
