//! Evaluation error.
//!
//! Several different error variants are defined. They are very analogous to
//! basic exception types from languages such as Python or Java.

use std::error::Error as StdError;
use std::fmt;

use super::Value;


/// Error that may have occurred during evaluation.
#[derive(Clone,Debug,PartialEq)]
pub enum Error {
    /// Invalid arguments.
    /// Indicates that actual arguments passed to an operation
    /// (like a function call) were invalid (for example, they had wrong types).
    Invalid(Mismatch),
    /// Other error with a custom message.
    Other(String),
}

impl Error {
    // TODO(xion): remove this legacy constructor after all usages of Error are fixed
    #[inline(always)]
    pub fn new(msg: &str) -> Error {
        Error::other(msg)
    }

    /// Create an Error that indicates an operation has received invalid arguments.
    /// It will not specify a valid argument types, however.
    #[inline]
    pub fn invalid(operation: &str, args: Vec<&Value>) -> Error {
        Error::Invalid(Mismatch::new(operation, args))
    }

    /// Create an Error that indicates an operation has received invalid arguments.
    /// The list of expected argument signatures is also required.
    #[inline]
    pub fn mismatch<T>(operation: &str, expected: Vec<Vec<T>>, actual: Vec<&Value>) -> Error
        where Type: From<T>
    {
        assert!(expected.len() > 0, "No expected argument signatures");
        Error::Invalid(Mismatch::against_many(
            operation,
            expected.into_iter()
                .map(|sig| sig.into_iter().map(Type::from).collect())
                .collect(),
            actual
        ))
    }

    #[inline(always)]
    pub fn other(msg: &str) -> Error {
        Error::Other(msg.to_owned())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Invalid(ref m) => write!(f, "Invalid arguments: {}", m),
            Error::Other(ref msg) => write!(f, "Eval error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Invalid(..) => "invalid arguments",
            Error::Other(..) => "evaluation error",
        }
    }

    #[inline(always)]
    fn cause(&self) -> Option<&StdError> {
        None
    }
}


// Structures for various error variants

/// Representation of a type.
/// For now, this is merely a type name.
pub type Type = String;

/// Type of a function or operator signature.
/// This is basically a list of argument type names it accepts.
pub type Signature = Vec<Type>;

/// Representation of a value.
/// For now, this is merely a Debug formatting of the Value.
pub type ValueRepr = String;

/// Mismatch error.
/// Indicates that values passed did not meet expectations.
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Mismatch {
    /// Name of the operation that caused the type mismatch.
    operation: String,
    /// List of expected signatures for the operation.
    expected: Vec<Signature>,
    /// Actual arguments passed.
    actual: Vec<(Type, ValueRepr)>,
}
impl Mismatch {
    #[inline(always)]
    pub fn new(operation: &str, args: Vec<&Value>) -> Mismatch {
        Mismatch::against_many(operation, Vec::new(), args)
    }

    #[inline(always)]
    pub fn against_one(operation: &str, expected: Signature, actual: Vec<&Value>) -> Mismatch {
        Mismatch::against_many(operation, vec![expected], actual)
    }

    #[inline(always)]
    pub fn against_many(operation: &str,
                        expected: Vec<Signature>, actual: Vec<&Value>) -> Mismatch {
        assert!(operation.len() > 0, "Empty operation");
        assert!(actual.len() > 0, "No actual arguments");
        Mismatch{
            operation: operation.to_owned(),
            expected: expected,
            actual: actual.into_iter()
                .map(|v| (Type::from(v.typename()), format!("{:?}", v))).collect(),
        }
    }
}
impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // format the operation identifier as either a function or an operator
        let mut operation = self.operation.clone();
        if operation.chars().all(|c| c == '_' || c.is_alphanumeric()) {
            if !operation.ends_with("()") {
                operation.push_str("()");
            }
        } else {
            operation = format!("`{}`", operation);
        }

        // if present, format the expected type signatures as separate lines
        let expected = match self.expected.len() {
            0 => "".to_owned(),
            1 => format!("expected ({}) but ", self.expected[0].join(", ")),
            _ => format!(
                "expected one of \n\t{}\nbut ", self.expected.iter()
                    .map(|sig| format!("({})", sig.join(", ")))
                    .collect::<Vec<_>>().join("\n")
            ),
        };

        // represent the actual values passed
        let actual_sep = if self.actual.len() > 2 { ", " } else { " and "};
        let actual = self.actual.iter()
            .map(|&(ref t, ref v)| format!("`{}` ({})", v, t))
            .collect::<Vec<_>>().join(actual_sep);

        write!(f, "{} {}got: {}", operation, expected, actual)
    }
}
