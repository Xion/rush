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


// Grammar utilities.

// TODO(xion): wait for https://github.com/Geal/nom/issues/149 to be addressed
// before using this macro in the grammar
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
    rest: many0!(pair!(
        map!(
            map_res!(
                // multispaced!(is_a!("*/")),
                delimited!(opt!(multispace), is_a!("*/"), opt!(multispace)),
                from_utf8),
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
    // of this branch, so trying to parse an atom as function will result
    // in incomplete input (because the pair of parentheses is "missing").
    // Using complete! forces the parses to interpret this IResult::Incomplete
    // as error (and thus try the `atom` branch) rather than bubble it up.
    complete!(chain!(
        name: identifier ~
        args: delimited!(
            delimited!(opt!(multispace), tag!("("), opt!(multispace)),
            args,
            delimited!(opt!(multispace), tag!(")"), opt!(multispace))
        ),
        move || {
            Box::new(
                FunctionCallNode{name: name, args: args}
            ) as Box<Eval>
        }
    )) | atom
));

/// args ::== expression (',' expression)*
named!(args( &[u8] ) -> Vec<Box<Eval>>, separated_list!(
    delimited!(opt!(multispace), tag!(","), opt!(multispace)),
    argument
));

// TODO(xion): support quoted strings
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    map_res!(identifier, |id: String| {
        id.parse::<ValueNode>().map(|node| Box::new(node) as Box<Eval>)
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
