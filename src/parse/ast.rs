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
            .collect::<Vec<String>>().join(", "))
    }
}


/// AST node representing the expression for creating a new object.
/// Objects are essentially hashmaps of strings to values.
///
/// The representation is a sequence of key-value pairs,
/// in their order of appearance in the expression.
pub struct ObjectNode {
    pub attributes: Vec<(Box<Eval>, Box<Eval>)>,
}

impl fmt::Debug for ObjectNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // The result is <Object: {$KEY: $VALUE}>, but braces have to be
        // escaped in format strings by doubling them: {{ -> {
        write!(fmt, "<Object: {{{}}}>", self.attributes.iter()
            .map(|&(ref k, ref v)| format!("{:?}: {:?}", k, v))
            .collect::<Vec<String>>().join(", "))
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


/// Associativity of a binary operator.
pub enum Associativity {
    /// Left associativity: a OP b OP c OP d === ((a OP b) OP c) OP d.
    /// In AST, this means first is a, and rest is [(OP, b), (OP, c), (OP, d)].
    Left,

    /// Right associativity: a OP b OP c OP d === a OP (b OP (c OP d)).
    ///
    /// In AST, this means first is d, rest is [(OP, c), (OP, b), (OP, a)],
    /// and the evaluation reverses order of arguments
    /// (compared to their position in expression source).
    Right,
}

/// AST node representing an operation involving binary operators
/// and their arguments.
///
/// Because of the way the operations are parsed, arbitrary length chains
/// of operations with the same priority (e.g. + and -) are represented
/// as one object.
///
pub struct BinaryOpNode {
    pub assoc: Associativity,
    pub first: Box<Eval>,
    pub rest: Vec<(String, Box<Eval>)>,
}

impl BinaryOpNode {
    pub fn new(assoc: Associativity,
               first: Box<Eval>, rest: Vec<(String, Box<Eval>)>) -> BinaryOpNode {
        BinaryOpNode{assoc: assoc, first: first, rest: rest}
    }
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self.assoc {
            Associativity::Left => format!(
                "{:?} {}", self.first, self.rest.iter()
                   .map(|&(ref op, ref arg)| format!("`{}` {:?}", op, arg))
                   .collect::<Vec<String>>().join(" ")
            ),
            Associativity::Right => unimplemented!(),
        };
        write!(fmt, "<Op {}>", repr)
    }
}


/// AST node representing a curried binary operator.
///
/// This is essenitally a function made out of said operator
/// by optionally providing left or right argument (or neither).
pub struct CurriedBinaryOpNode  {
    pub op: String,
    pub left: Option<Box<Eval>>,
    pub right: Option<Box<Eval>>,
}

impl CurriedBinaryOpNode {
    pub fn with_none(op: String) -> CurriedBinaryOpNode {
        CurriedBinaryOpNode{op: op, left: None, right: None}
    }
    pub fn with_left(op: String, arg: Box<Eval>) -> CurriedBinaryOpNode {
        CurriedBinaryOpNode{op: op, left: Some(arg), right: None}
    }
    pub fn with_right(op: String, arg: Box<Eval>) -> CurriedBinaryOpNode {
        CurriedBinaryOpNode{op: op, left: None, right: Some(arg)}
    }
}

impl fmt::Debug for CurriedBinaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<CurriedOp ({}{}{})>",
            self.left.as_ref().map(|l| format!("{:?} ", l)).unwrap_or(String::new()),
            self.op,
            self.right.as_ref().map(|r| format!(" {:?}", r)).unwrap_or(String::new()))
    }
}


/// Index used for subscripting.
pub enum Index {
    /// Point index, referring to a single element.
    Point(Box<Eval>),

    /// Range index, referring to a half-open range of elements.
    /// The upper bound is exclusive.
    Range(Option<Box<Eval>>, Option<Box<Eval>>),
}

impl fmt::Debug for Index {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Index::Point(ref p) => write!(fmt, "{:?}", p),
            Index::Range(ref l, ref r) => write!(fmt, "{}:{}",
                l.as_ref().map(|p| format!("{:?}", p)).unwrap_or(String::new()),
                r.as_ref().map(|p| format!("{:?}", p)).unwrap_or(String::new())),
        }
    }
}

/// AST node representing an operation of taking a subscript of an object
/// (also referred to as "indexing").
///
/// The object is commonly an array or a string.
pub struct SubscriptNode {
    pub object: Box<Eval>,
    pub index: Index,
}

impl fmt::Debug for SubscriptNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Index: {:?}[{:?}]>", self.object, self.index)
    }
}


/// AST node representing a call to, or an application of,
/// a function with/to given arguments.
///
/// The exact function the expression resolves to
/// depends on the context passed during evaluation.
pub struct FunctionCallNode {
    pub func: Box<Eval>,
    pub args: Vec<Box<Eval>>,
}

impl fmt::Debug for FunctionCallNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let args = self.args.iter()
            .map(|arg| format!("{:?}", arg))
            .collect::<Vec<String>>().join(",");
        write!(fmt, "<Call: {:?}({})>", self.func, args)
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
