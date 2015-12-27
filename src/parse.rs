//! Parser code for the expression syntax.

use std::str::from_utf8;

use nom::{alphanumeric, multispace, IResult, Needed, Err, ErrorKind};

use ast::*;
use eval::{Eval, Value};


/// Parse given exprssion, returning the AST that represents it.
pub fn parse(input: &str) -> Box<Eval> {
    match expression(input.trim().as_bytes()) {
        IResult::Done(_, node) => node,

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

/// argument ::== term [('+' | '-') argument]
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    left: term ~
    maybe_right: opt!(chain!(
        multispace? ~
        op: map_res!(is_a!("+-"), from_utf8) ~
        multispace? ~
        argument: argument,
        || { (op, argument) }
    )),
    move || {
        match maybe_right {
            Some((op, right)) => Box::new(
                BinaryOpNode{op: op.to_string(),
                             left: left,
                             right: right}
            ) as Box<Eval>,
            None => left
        }
    }
));

/// term ::== factor [('*' | '/') term]
named!(term( &[u8] ) -> Box<Eval>, chain!(
    left: factor ~
    maybe_right: opt!(chain!(
        multispace? ~
        op: map_res!(is_a!("*/"), from_utf8) ~
        multispace? ~
        term: term,
        || { (op, term) }
    )),
    move || {
        match maybe_right {
            Some((op, right)) => Box::new(
                BinaryOpNode{op: op.to_string(),
                             left: left,
                             right: right}
            ) as Box<Eval>,
            None => left
        }
    }
));

/// factor ::== atom [function_args]
named!(factor( &[u8] ) -> Box<Eval>, chain!(
    atom: atom ~
    function_args: opt!(function_args),
    move || {
        match function_args {
            Some(args) => Box::new(
                FunctionCallNode{
                    name: atom.value.as_string().unwrap(),
                    args: args,
                }
            ) as Box<Eval>,
            None => Box::new(atom) as Box<Eval>,
        }
    }
));

// TODO(xion): support quoted strings
named!(atom( &[u8] ) -> ValueNode, chain!(
    value: map_res!(alt!(tag!("_") | alphanumeric), from_utf8),
    || { value.parse::<ValueNode>().unwrap() }
));

/// function_args ::== '(' (expression ',')* ')'
named!(function_args( &[u8] ) -> Vec<Box<Eval>>, chain!(
    is_a!("(") ~
    args: many0!(chain!(
        // TODO(xion): disallow trailing comma (need to explicitly handle
        // first or last arg for that)
        multispace? ~ arg: expression ~ multispace? ~ is_a!(","),
        || { arg }
    )) ~
    multispace? ~ is_a!(")"),
    || { args }
));
