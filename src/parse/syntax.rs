//! Expression syntax.
//! Uses nom's parser combinators to define the grammar.
//!
//! alongside the parsing code that assembles the AST.

use std::str::from_utf8;

use nom::{self, alpha, alphanumeric, multispace, IResult};

use ast::*;
use eval::Eval;


// TODO(xion): switch from parsers expecting &[u8] to accepting &str;
// this will get rid of the hack in float_literal()


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

// TODO(xion): correct parsing of floating point numbers (it's broken now)
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    map_res!(alt!(identifier | float_literal | int_literal | string_literal), |id: String| {
        id.parse::<AtomNode>().map(|node| Box::new(node) as Box<Eval>)
    }) |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

named!(identifier( &[u8] ) -> String, alt!(
    // TODO(xion): typed underscore vars (_i, _f)
    string!(tag!("_")) |
    map_res!(
        pair!(alpha, many0!(alphanumeric)), |(first, rest): (_, Vec<&[u8]>)| {
            let mut rest = rest;
            rest.insert(0, first);
            from_utf8(&rest.concat()[..]).map(str::to_string)
        }
    )
));

const DIGITS: &'static str = "0123456789";
named!(int_literal( &[u8] ) -> String, map_res!(
    pair!(is_a!(&DIGITS[1..]), many0!(is_a!(DIGITS))),
    |(first, rest): (_, Vec<&[u8]>)| {
        let mut rest = rest;
        rest.insert(0, first);
        from_utf8(&rest.concat()[..]).map(str::to_string)
    }
));

const FLOAT_REGEX: &'static str = r"(0|[1-9][0-9]*)\.[0-9]+(e[+-]?[1-9][0-9]*)?";
fn float_literal(input: &[u8]) -> IResult<&[u8], String> {
    let (_, input) = try_parse!(input, expr_res!(from_utf8(input)));

    // TODO(xion): use re_match_static! when regexp_macros feature
    // can be used in stable Rust
    let result = re_match!(input, FLOAT_REGEX);

    // This match has to be explicit (rather than try_parse! etc.)
    // because of the silly IResult::Error branch, which is seemingly no-op
    // but it forces the result to be of correct type (nom::Err<&[u8]>
    // rather than nom::Err<&str> returned by regex match parser).
    // TODO(xion): consider switching all parsers to &str->&str
    // to avoid this hack and the various map_res!(..., from_utf8) elsewhere
    match result {
        IResult::Done(rest, parsed) =>
            IResult::Done(rest.as_bytes(), parsed.to_string()),
        IResult::Incomplete(i) => IResult::Incomplete(i),
        IResult::Error(nom::Err::Code(e)) => IResult::Error(nom::Err::Code(e)),
        _ => panic!("unexpected IResult from re_match!"),
    }
}

// TODO(xion): quote escaping
named!(string_literal( &[u8] ) -> String, string!(
    preceded!(tag!("\""), take_until_and_consume!("\""))
));
