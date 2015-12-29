//! Parser code for the expression syntax.

use std::str::from_utf8;

use nom::{alphanumeric, multispace, IResult, Needed, Err};

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


macro_rules! multispaced (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        delimited!(opt!(multispace), $submac!($i, $($args)*), opt!(multispace));
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
            map_res!(
                // multispaced!(is_a!("+-")),
                delimited!(opt!(multispace), is_a!("+-"), opt!(multispace)),
                from_utf8),
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
    rest: many0!(chain!(
        op: map_res!(
            delimited!(opt!(multispace), is_a!("*/"), opt!(multispace)),
            from_utf8) ~
        factor: factor,
        || { (op.to_string(), factor) }
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
    chain!(
        name: identifier ~
        args: chain!(
            is_a!("(") ~ multispace? ~ args: args ~ multispace? ~  is_a!(")"),
            || { args }
        ),
        move || {
            Box::new(
                FunctionCallNode{name: name, args: args}
            ) as Box<Eval>
        }
    ) | atom
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

// TODO(xion): support quoted strings
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    map!(identifier, |id: String| {
        Box::new(id.parse::<ValueNode>().unwrap()) as Box<Eval>
    }) |
    delimited!(
        delimited!(opt!(multispace), tag!("("), opt!(multispace)),
        expression,
        delimited!(opt!(multispace), tag!(")"), opt!(multispace))
    )
));

// TODO(xion): typed underscore vars (_i, _f)
named!(identifier( &[u8] ) -> String, map!(
    map_res!(alt!(tag!("_") | alphanumeric), from_utf8),
    str::to_string
));
