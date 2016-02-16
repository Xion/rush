//! Parser code for the expression syntax.

mod error;
mod syntax;

pub mod ast;
pub use self::error::ParseError;


use std::str::from_utf8;

use nom::IResult;

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
                    Ok(i) => ParseError::Excess(i.to_owned()),
                    // TODO(xion): bubble the error from the various
                    // from_utf8 calls in gramamar rules
                    _ => ParseError::Corrupted,
                })
            }
        },
        IResult::Incomplete(needed) => Err(ParseError::Incomplete(needed)),
        IResult::Error(_) => Err(ParseError::Invalid),
    }
}
