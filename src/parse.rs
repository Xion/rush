//! Parser code for the expression syntax.

use std::str::from_utf8;

use nom::{alphanumeric, multispace, IResult, Needed, Err, ErrorKind};

use ast::*;
use eval::Eval;


/// Parse given exprssion, returning the AST that represents it.
pub fn parse(input: &str) -> Box<Eval> {
    match expression(input.trim().as_bytes()) {
        IResult::Done(input, node) => {
            if !input.is_empty() {
                panic!("excess input: {}", from_utf8(input).unwrap());
            }
            node
        },

        IResult::Incomplete(Needed::Size(c)) => {
            panic!("incomplete input, need {} more bytes", c);
        },
        IResult::Incomplete(Needed::Unknown) => {
            panic!("incomplete input");
        }

        // TODO(xion): parse the error value and convert to custom error type,
        // returning a Result<...> from this function
        IResult::Error(e) => panic!("parse error: {:?}", e),
    }
}


// Grammar definition.

named!(expression( &[u8] ) -> Box<Eval>, chain!(e: argument, || { e }));

/// argument ::== term (('+' | '-') argument)*
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    first: term ~
    rest: many0!(chain!(
        multispace? ~
        op: map_res!(is_a!("+-"), from_utf8) ~
        multispace? ~
        argument: argument,
        || { (op.to_string(), argument) }
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// term ::== factor (('*' | '/') term)*
named!(term( &[u8] ) -> Box<Eval>, chain!(
    first: factor ~
    rest: many0!(chain!(
        multispace? ~
        op: map_res!(is_a!("+-"), from_utf8) ~
        multispace? ~
        term: term,
        || { (op.to_string(), term) }
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// factor ::== atom ['(' args ')']
named!(factor( &[u8] ) -> Box<Eval>, chain!(
    atom: atom ~
    maybe_args: opt!(chain!(
        is_a!("(") ~ multispace? ~ args: args ~ multispace? ~  is_a!(")"),
        || { args }
    )),
    move || {
        match maybe_args {
            Some(args) => Box::new(
                FunctionCallNode{
                    // TODO(xion): better error handling
                    name: atom.value.as_string().unwrap(),
                    args: args,
                }
            ) as Box<Eval>,
            None => Box::new(atom) as Box<Eval>,
        }
    }
));

// TODO(xion): support quoted strings
// TODO(xion): support parenthesized expressions
named!(atom( &[u8] ) -> ValueNode, chain!(
    value: map_res!(alt!(tag!("_") | alphanumeric), from_utf8),
    || { value.parse::<ValueNode>().unwrap() }
));

/// args ::== expression (',' expression)*
named!(args( &[u8] ) -> Vec<Box<Eval>>, chain!(
    first: expression ~
    rest: many0!(chain!(
        multispace? ~ is_a!(",") ~ multispace? ~
        arg: expression,
        || { arg }
    )),
    move || {
        let mut rest = rest;
        rest.insert(0, first);
        rest
    }
));
