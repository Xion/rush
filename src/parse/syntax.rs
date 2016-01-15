//! Expression syntax.
//! Uses nom's parser combinators to define the grammar.
//!
//! alongside the parsing code that assembles the AST.

use std::str::from_utf8;

use nom::{self, alpha, alphanumeric, multispace, IResult};

use ast::*;
use eval::{Eval, Value};


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


// Grammar constants.

const ADDITIVE_BINARY_OPS: &'static str = "+-";
const MULTIPLICATIVE_BINARY_OPS: &'static str = "*/%";
const UNARY_OPS: &'static str = "+-!";

const RESERVED_WORDS: &'static [&'static str] = &[
    "const", "do", "else", "false", "for", "if", "let", "true", "while",
];

const DIGITS: &'static str = "0123456789";
const FLOAT_REGEX: &'static str = r"(0|[1-9][0-9]*)\.[0-9]+(e[+-]?[1-9][0-9]*)?";

const UNDERSCORE_SUFFIXES: &'static str = "bifs";


// Grammar definition.

/// Root symbol of the grammar.
named!(pub expression( &[u8] ) -> Box<Eval>, chain!(e: argument, || { e }));

/// argument ::== term (ADDITIVE_BIN_OP term)*
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    first: term ~
    rest: many0!(pair!(
        string!(multispaced!(is_a!(ADDITIVE_BINARY_OPS))),
        term
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// term ::== factor (MULT_BIN_OP factor)*
named!(term( &[u8] ) -> Box<Eval>, chain!(
    first: factor ~
    rest: many0!(pair!(
        string!(multispaced!(is_a!(MULTIPLICATIVE_BINARY_OPS))),
        factor
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// factor ::== [UNARY_OP] (identifier '(' args ')' | atom)
named!(factor( &[u8] ) -> Box<Eval>, map!(
    pair!(
        opt!(string!(multispaced!(is_a!(UNARY_OPS)))),
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

/// atom ::== BOOLEAN | SYMBOL | FLOAT | INTEGER | STRING | '(' expression ')'
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    bool_value | symbol_value | float_value | int_value | string_value |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

named!(bool_value( &[u8] ) -> Box<Eval>, alt!(
    tag!("false") => { |_| Box::new(AtomNode::new(Value::Boolean(false))) } |
    tag!("true") => { |_| Box::new(AtomNode::new(Value::Boolean(true))) }
));

named!(symbol_value( &[u8] ) -> Box<Eval>, map!(identifier, |value: String| {
    Box::new(AtomNode::new(Value::Symbol(value)))
}));
named!(identifier( &[u8] ) -> String, alt!(
    map!(
        pair!(string!(tag!("_")), opt!(string!(is_a!(UNDERSCORE_SUFFIXES)))),
        |(uscore, maybe_suffix) : (String, Option<String>)| {
            let mut result = uscore;
            result.push_str(&maybe_suffix.unwrap_or(String::new()));
            result
        }
    ) |
    map_res!(
        pair!(alpha, many0!(alphanumeric)), |(first, rest): (_, Vec<&[u8]>)| {
            let mut rest = rest;
            rest.insert(0, first);
            // TODO(xion): better error handling for the reserved word case
            // (note that map_res! generally discards errors so we may have
            // to use fix_error!, add_error!, or error!)
            from_utf8(&rest.concat()[..])
                .map_err(|_| ())
                .and_then(|s| { if RESERVED_WORDS.contains(&s) { Err(()) }
                                else { Ok(s) } })
                .map(str::to_string)
        }
    )
));

named!(int_value( &[u8] ) -> Box<Eval>, map_res!(int_literal, |value: String| {
    value.parse::<i64>().map(|i| Box::new(AtomNode::new(Value::Integer(i))))
}));
named!(int_literal( &[u8] ) -> String, alt!(
    map_res!(
        pair!(is_a!(&DIGITS[1..]), many0!(is_a!(DIGITS))),
        |(first, rest): (_, Vec<&[u8]>)| {
            let mut rest = rest;
            rest.insert(0, first);
            from_utf8(&rest.concat()[..]).map(str::to_string)
        }
    ) |
    string!(tag!("0"))
));

named!(float_value( &[u8] ) -> Box<Eval>, map_res!(float_literal, |value: String| {
    value.parse::<f64>().map(|f| Box::new(AtomNode::new(Value::Float(f))))
}));
fn float_literal(input: &[u8]) -> IResult<&[u8], String> {
    let (_, input) = try_parse!(input, expr_res!(from_utf8(input)));

    // TODO(xion): use *_static! variant when regexp_macros feature
    // can be used in stable Rust
    let result = re_find!(input, FLOAT_REGEX);

    // This match has to be explicit (rather than try_parse! etc.)
    // because of the silly IResult::Error branch, which is seemingly no-op
    // but it forces the result to be of correct type (nom::Err<&[u8]>
    // rather than nom::Err<&str> returned by regex parser).
    // TODO(xion): consider switching all parsers to &str->&str
    // to avoid this hack and the various map_res!(..., from_utf8) elsewhere
    match result {
        IResult::Done(rest, parsed) =>
            IResult::Done(rest.as_bytes(), parsed.to_string()),
        IResult::Incomplete(i) => IResult::Incomplete(i),
        IResult::Error(nom::Err::Code(e)) => IResult::Error(nom::Err::Code(e)),
        _ => panic!("unexpected IResult from re_find!"),
    }
}

// TODO(xion): quote escaping
named!(string_value( &[u8] ) -> Box<Eval>, map!(string_literal, |value: String| {
    Box::new(AtomNode::new(Value::String(value)))
}));
named!(string_literal( &[u8] ) -> String, string!(
    preceded!(tag!("\""), take_until_and_consume!("\""))
));
