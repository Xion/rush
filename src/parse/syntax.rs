//! Expression syntax.
//! Uses nom's parser combinators to define the grammar.
//!
//! alongside the parsing code that assembles the AST.

use std::str::from_utf8;

use nom::{alphanumeric, multispace};

use ast::*;
use eval::Eval;


// Grammar utilities.

/// Make the underlying parser assume UTF8-encoded input
/// and output String objects.
macro_rules! string {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i, map_res!($submac!($($args)*), from_utf8), str::to_string);
    );
    ($i:expr, $f:expr) => (
        string!($i, call!($f));
    );
}

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

/// Root symbol of the grammar.
named!(pub expression( &[u8] ) -> Box<Eval>, chain!(e: argument, || { e }));

/// argument ::== term (('+' | '-') term)*
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    first: term ~
    rest: many0!(pair!(
        string!(multispaced!(is_a!("+-"))),
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
        string!(multispaced!(is_a!("*/"))),
        factor
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// factor ::== ['+'|'-'] (identifier '(' args ')' | atom)
named!(factor( &[u8] ) -> Box<Eval>, map!(
    pair!(
        opt!(string!(multispaced!(is_a!("+-")))),
        alt!(
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
        )
    ),
    |(maybe_op, factor): (_, Box<Eval>)| match maybe_op {
        Some(op) => Box::new(UnaryOpNode{op: op, arg: factor}) as Box<Eval>,
        None => factor,
    }
));

/// args ::== expression (',' expression)*
named!(args( &[u8] ) -> Vec<Box<Eval>>,
       separated_list!(multispaced!(tag!(",")), argument));

// TODO(xion): support quoted strings
// TODO(xion): correct parsing of floating point numbers (it's broken now)
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    map_res!(identifier, |id: String| {
        id.parse::<AtomNode>().map(|node| Box::new(node) as Box<Eval>)
    }) |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

// TODO(xion): typed underscore vars (_i, _f)
named!(identifier( &[u8] ) -> String, string!(
    alt!(tag!("_") | alphanumeric)
));
