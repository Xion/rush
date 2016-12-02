//! Parser for the literal ("atomic") values, like numbers and strings.

use std::f64;
use std::str::from_utf8;

use nom::{self, alpha, alphanumeric, IResult};
use regex::Regex;

use eval::{Eval, Value};
use eval::value::{FloatRepr, IntegerRepr, RegexRepr, StringRepr};
use parse::ast::{ArrayNode, ObjectNode, ScalarNode};
use super::structure::expression;


// TODO(xion): switch from parsers expecting &[u8] to accepting &str;
// this will get rid of the hack in float_literal() and possibly other cruft


const RESERVED_WORDS: &'static [&'static str] = &[
    "const", "do", "else", "false", "for", "if", "let", "true", "while",
];

const DIGITS: &'static str = "0123456789";
const FLOAT_REGEX: &'static str = r"(0|[1-9][0-9]*)\.[0-9]+([eE][+-]?[1-9][0-9]*)?";
const ESCAPE: &'static str = "\\";

const UNDERSCORE_SUFFIXES: &'static str = "bifs";


/// identifier ::== ('_' SUFFIX?) | (ALPHA ALPHANUMERIC*)
named!(pub identifier( &[u8] ) -> String, alt!(
    string!(seq!(tag!("_"), maybe!(char_of!(UNDERSCORE_SUFFIXES)))) |
    map_res!(string!(seq!(alpha, many0!(alphanumeric))), |ident: String| {
        {
            let id: &str = &ident;
            if RESERVED_WORDS.contains(&id) {
                // TODO(xion): better error handling for the reserved word case
                // (note that map_res! generally discards errors so we may have
                // to use fix_error!, add_error!, or error!)
                return Err(());
            }
        }
        Ok(ident)
    })
));

/// atom ::== PRIMITIVE | '(' expression ')'
/// PRIMITIVE ::== NIL | OBJECT | ARRAY | BOOLEAN | FLOAT | INTEGER | SYMBOL | REGEX | STRING
named!(pub atom( &[u8] ) -> Box<Eval>, alt!(
    //
    // Note that order of those branches matters.
    // Literals that have special case'd identifiers as valid values
    // -- like floats with their NaN -- have to be before symbols!
    //
    nil_value |
    object_value | array_value |
    bool_value | float_value | int_value | symbol_value |
    regex_value | string_value |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

named!(nil_value( &[u8] ) -> Box<Eval>, map!(tag!("nil"), |_| {
    Box::new(ScalarNode::from(Value::Empty))
}));

/// OBJECT ::== '{' [expression ':' expression] (',' expression ':' expression)* '}'
named!(object_value( &[u8] ) -> Box<Eval>, map!(
    delimited!(
        multispaced!(tag!("{")),
        separated_list!(
            multispaced!(tag!(",")),
            separated_pair!(expression, multispaced!(tag!(":")), expression)
        ),
        multispaced!(tag!("}"))
    ),
    |attrs| { Box::new(ObjectNode::new(attrs)) }
));

/// ARRAY ::== '[' [expression] (',' expression)* ']'
named!(array_value( &[u8] ) -> Box<Eval>, map!(
    delimited!(
        multispaced!(tag!("[")),
        separated_list!(multispaced!(tag!(",")), expression),
        multispaced!(tag!("]"))
    ),
    |items| { Box::new(ArrayNode::new(items)) }
));

named!(bool_value( &[u8] ) -> Box<Eval>, alt!(
    tag!("false") => { |_| Box::new(ScalarNode::from(false)) } |
    tag!("true") => { |_| Box::new(ScalarNode::from(true)) }
));

named!(symbol_value( &[u8] ) -> Box<Eval>, map!(identifier, |value: String| {
    Box::new(ScalarNode::from(Value::Symbol(value)))
}));

named!(int_value( &[u8] ) -> Box<Eval>, map_res!(int_literal, |value: String| {
    value.parse::<IntegerRepr>().map(ScalarNode::from).map(Box::new)
}));
named!(int_literal( &[u8] ) -> String, string!(alt!(
    seq!(char_of!(&DIGITS[1..]), many0!(char_of!(DIGITS))) | tag!("0")
)));

named!(float_value( &[u8] ) -> Box<Eval>, alt!(
    tag!("Inf") => { |_| Box::new(ScalarNode::from(f64::INFINITY as FloatRepr)) } |
    tag!("NaN") => { |_| Box::new(ScalarNode::from(f64::NAN as FloatRepr)) } |
    map_res!(float_literal, |value: String| {
        value.parse::<FloatRepr>().map(ScalarNode::from).map(Box::new)
    })
));
fn float_literal(input: &[u8]) -> IResult<&[u8], String> {
    let (_, input) = try_parse!(input, expr_res!(from_utf8(input)));

    // TODO(xion): use *_static! variant when regexp_macros feature
    // can be used in stable Rust
    let regex = "^".to_owned() + FLOAT_REGEX;  // 'coz we want immediate match
    let result = re_find!(input, &regex);

    // This match has to be explicit (rather than try_parse! etc.)
    // because of the silly IResult::Error branch, which is seemingly no-op
    // but it forces the result to be of correct type (nom::Err<&[u8]>
    // rather than nom::Err<&str> returned by regex parser).
    // TODO(xion): consider switching all parsers to &str->&str
    // to avoid this hack and the various map_res!(..., from_utf8) elsewhere
    match result {
        IResult::Done(rest, parsed) =>
            IResult::Done(rest.as_bytes(), String::from(parsed)),
        IResult::Incomplete(i) => IResult::Incomplete(i),
        IResult::Error(nom::Err::Code(e)) => IResult::Error(nom::Err::Code(e)),
        r => unreachable!("unexpected result from parsing float: {:?}", r),
    }
}

named!(regex_value( &[u8] ) -> Box<Eval>, map!(regex_literal, |value: RegexRepr| {
    Box::new(ScalarNode::from(value))
}));
fn regex_literal(input: &[u8]) -> IResult<&[u8], Regex> {
    let (mut input, _) = try_parse!(input, tag!("/"));

    // consume chacters until the closing slash
    let mut r = String::new();
    loop {
        let (rest, chunk) = try_parse!(input,
                                       string!(take_until_and_consume!("/")));
        r.push_str(&chunk);

        input = rest;
        if input.is_empty() {
            break;
        }

        // Try to parse what we've got so far as a regex;
        // if it succeeds, then this is our result.
        // Note that this will handle escaping of the slash
        // (to make it literal part of the regex) through character class [/],
        // since unterminated square bracket won't parse as regex.
        if let Ok(regex) = Regex::new(&r) {
            return IResult::Done(input, regex);
        }

        r.push('/');
    }

    // If we exhausted the input, then whatever we've accumulated so far
    // may still be a valid regex, so try to parse it.
    expr_res!(input, Regex::new(&r))
}

named!(string_value( &[u8] ) -> Box<Eval>, map!(string_literal, |value: StringRepr| {
    Box::new(ScalarNode::from(Value::String(value)))
}));
#[allow(single_char_pattern)]
fn string_literal(input: &[u8]) -> IResult<&[u8], String> {
    let (mut input, _) = try_parse!(input, tag!("\""));

    // consume characters until the closing double quote
    let mut s = String::new();
    loop {
        let (rest, chunk) = try_parse!(input,
                                       string!(take_until_and_consume!("\"")));
        input = rest;

        if chunk.is_empty() {
            break;
        }
        s.push_str(&chunk);

        // however, if the quote was escaped, the string continues beyond it
        // and requires parsing of another chunk
        if !chunk.ends_with(ESCAPE) {
            break;
        }
        s.push('"');
    }

    // replace the escape sequences with corresponding characters
    s = s.replace(&format!("{}\"", ESCAPE), "\"");  // double quotes
    s = s.replace(&format!("{}n", ESCAPE), "\n");
    s = s.replace(&format!("{}r", ESCAPE), "\r");
    s = s.replace(&format!("{}t", ESCAPE), "\t");
    s = s.replace(&format!("{}{}", ESCAPE, ESCAPE), ESCAPE);  // must be last

    IResult::Done(input, s)
}
