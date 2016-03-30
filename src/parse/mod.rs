//! Parser code for the expression syntax.

mod error;
mod syntax;

pub mod ast;
pub use self::error::Error;


use std::str::from_utf8;

use nom::IResult;

use eval::Eval;
use self::syntax::expression;


/// Parse given expression, returning the AST that represents it.
pub fn parse(input: &str) -> Result<Box<Eval>, Error> {
    if input.is_empty() {
        return Err(Error::Empty);
    }

    match expression(input.trim().as_bytes()) {
        IResult::Done(input, node) => {
            if input.is_empty() {
                Ok(node)
            } else {
                Err(match from_utf8(input) {
                    Ok(i) => Error::Excess(i.to_owned()),
                    // TODO(xion): bubble the error from the various
                    // from_utf8 calls in grammar rules
                    _ => Error::Corrupted,
                })
            }
        },
        IResult::Incomplete(needed) => Err(Error::Incomplete(needed)),
        IResult::Error(_) => Err(Error::Invalid),
    }
}
