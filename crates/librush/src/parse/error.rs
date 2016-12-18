//! Parse error type.

use std::error::Error as StdError;
use std::fmt;

use nom::Needed;


/// Error from parsing an expression.
#[derive(Clone,Debug)]
pub enum Error {
    /// Empty input.
    Empty,
    /// Not an UTF8 input.
    Corrupted,
    /// Parse error (input doesn't follow valid expression syntax).
    // TODO(xion): include more information, like the offending character index
    Invalid,
    /// Extra input beyond what's allowed by expression syntax.
    Excess(String),
    /// Unexpected end of input.
    Incomplete(Needed),
}

impl Error {
    /// Whether the error can be interpreted as simple syntax error.
    pub fn is_syntax(&self) -> bool {
        match *self {
            Error::Empty | Error::Corrupted => false,
            _ => true
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Excess(ref s) =>
                write!(f, "expected end of expression, but found `{}`", s),
            Error::Incomplete(_) =>
                write!(f, "unexpected end of expression"),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Empty => "empty expression",
            Error::Corrupted => "non-UTF8 expression",
            Error::Invalid => "syntax error",
            Error::Excess(_) => "unexpected expression",
            Error::Incomplete(_) => "unexpected end of expression",
        }
    }

    #[allow(match_same_arms)]
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Empty |
            Error::Excess(_) |
            Error::Incomplete(_) => None,
            // TODO(xion): for the rest, we could store or recreate
            // the original Error to return it as cause here
            _ => None,
        }
    }
}
