//! Parser code for the expression syntax.

mod syntax;


use std::error::Error;
use std::fmt;
use std::str::from_utf8;

use nom::{IResult, Needed};

use eval::Eval;
use self::syntax::expression;


/// Parse given expression, returning the AST that represents it.
pub fn parse(input: &str) -> Result<Box<Eval>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }
    match expression(input.trim().as_bytes()) {
        IResult::Done(input, node) => {
            if input.is_empty() {
                Ok(node)
            } else {
                Err(match from_utf8(input) {
                    Ok(i) => ParseError::Excess(i.to_string()),
                    // TODO(xion): bubble the error from the various
                    // from_utf8 calls in gramamr rules
                    _ => ParseError::Corrupted,
                })
            }
        },
        IResult::Incomplete(needed) => Err(ParseError::Incomplete(needed)),
        IResult::Error(_) => Err(ParseError::Invalid),
    }
}

/// Error from parsing an expression.
#[derive(Debug)]
pub enum ParseError {
    /// Empty input.
    Empty,
    /// Not an UTF8 input.
    Corrupted,
    /// Parse error (input doesn't follow valid expression syntax).
    // TODO(xion): include more information, like the offending chracter index
    Invalid,
    /// Extra input beyond what's allowed by expression syntax.
    Excess(String),
    /// Unexpected end of input.
    Incomplete(Needed),
}

impl ParseError {
    /// Whether the error can be interpreted as simple syntax error.
    pub fn is_syntax(self) -> bool {
        match self {
            ParseError::Empty | ParseError::Corrupted => false,
            _ => true
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        // TODO(xion): error descriptions
        "Parse error"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Empty |
            ParseError::Excess(_) |
            ParseError::Incomplete(_) => None,
            // TODO(xion): for the rest, we could store or recreate
            // the original Error to return it as cause here
            _ => None,
        }
    }
}
