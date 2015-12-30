//! Parser code for the expression syntax.

use std::error::Error;
use std::fmt;
use std::str::from_utf8;

use nom::{alphanumeric, multispace, IResult, ErrorKind, Needed};

use ast::*;
use eval::Eval;


/// Parse given exprssion, returning the AST that represents it.
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


// Grammar utilities.

/// Parses values that are optionally surrounded by arbitrary number of
/// any of the whitespace characters.
macro_rules! multispaced (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        delimited!($i, opt!(multispace), $submac!($($args)*), opt!(multispace));
    );
    ($i:expr, $f:expr) => (
        multispaced!($i, call!($f));
    );
);


// Grammar definition.

named!(expression( &[u8] ) -> Box<Eval>, chain!(e: argument, || { e }));

/// argument ::== term (('+' | '-') term)*
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    first: term ~
    rest: many0!(pair!(
        map!(
            map_res!(multispaced!(is_a!("+-")), from_utf8),
            str::to_string
        ),
        term
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// term ::== factor (('*' | '/') factor)*
named!(term( &[u8] ) -> Box<Eval>, chain!(
    first: factor ~
    rest: many0!(pair!(
        map!(
            map_res!(multispaced!(is_a!("*/")), from_utf8),
            str::to_string
        ),
        factor
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// factor ::== identifier '(' args ')' | atom
named!(factor( &[u8] ) -> Box<Eval>, alt!(
    // complete!(...) is necessary because `atom` branch below can be a prefix
    // of this branch, so trying to parse an atom as function call will result
    // in incomplete input (because the pair of parentheses is "missing").
    // Using complete! forces the parses to interpret this IResult::Incomplete
    // as error (and thus try the `atom` branch) rather than bubble it up.
    complete!(chain!(
        name: identifier ~
        args: delimited!(multispaced!(tag!("(")), args, multispaced!(tag!(")"))),
        move || {
            Box::new(
                FunctionCallNode{name: name, args: args}
            ) as Box<Eval>
        }
    )) | atom
));

/// args ::== expression (',' expression)*
named!(args( &[u8] ) -> Vec<Box<Eval>>,
       separated_list!(multispaced!(tag!(",")), argument));

// TODO(xion): support quoted strings
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    map_res!(identifier, |id: String| {
        id.parse::<ValueNode>().map(|node| Box::new(node) as Box<Eval>)
    }) |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

// TODO(xion): typed underscore vars (_i, _f)
named!(identifier( &[u8] ) -> String, map!(
    map_res!(alt!(tag!("_") | alphanumeric), from_utf8),
    str::to_string
));
