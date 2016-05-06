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
    /// Type error.
    /// Indicates that a type signature of actual arguments passed to an operation
    /// (like a function call) differed from the list of accepted ones.
    Type(TypeMismatch),
    /// Value error.
    /// Indicates that the value(s) passed to an operation were invalid.
    Value(Vec<Value>),

    /// Other error with a custom message.
    Other(String),
}

impl Error {
    // TODO(xion): remove this legacy constructor after all usages of Error are fixed
    #[inline(always)]
    pub fn new(msg: &str) -> Error {
        Error::other(msg)
    }

    pub fn type_mismatch(operation: &str, expected: Vec<Vec<&str>>, actual: Vec<&str>) -> Error {
        Error::Type(TypeMismatch::against_many(
            operation.to_owned(),
            expected.into_iter()
                .map(|sig| sig.into_iter().map(String::from).collect())
                .collect(),
            actual.into_iter().map(String::from).collect()
        ))
    }
    pub fn other(msg: &str) -> Error {
        Error::Other(msg.to_owned())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Type(ref mismatch) => write!(f, "Type mismatch: {}", mismatch),
            Error::Value(ref value) => {
                // TODO(xion): different message for one and many values
                write!(f, "Unexpected value(s): {:?}", value)
            },
            Error::Other(ref msg) => write!(f, "Eval error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Type(..) => "type mismatch",
            Error::Value(..) => "invalid value",
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

/// Type mismatch error.
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct TypeMismatch {
    /// Name of the operation that caused the type mismatch.
    operation: String,
    /// List of expected signatures for the operation.
    expected: Vec<Signature>,
    /// Actual type signature passed.
    actual: Signature,
}
impl TypeMismatch {
    #[inline(always)]
    pub fn against_one(operation: String, expected: Signature, actual: Signature) -> TypeMismatch {
        TypeMismatch::against_many(operation, vec![expected], actual)
    }

    #[inline(always)]
    pub fn against_many(operation: String,
                        expected: Vec<Signature>, actual: Signature) -> TypeMismatch {
        assert!(operation.len() > 0, "Empty operation");
        assert!(expected.len() > 0, "Empty list of expected type signatures");
        TypeMismatch{operation: operation, expected: expected, actual: actual}
    }
}
impl fmt::Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn format_signature(sig: &Signature) -> String {
            format!("({})", sig.join(", "))
        }
        write!(f, "invalid arguments for `{}` - expected one of\n{}\ngot {}:",
            self.operation,
            self.expected.iter().map(format_signature).collect::<Vec<_>>().join("\n"),
            format_signature(&self.actual))
    }
}
