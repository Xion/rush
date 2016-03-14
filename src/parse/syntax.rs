//! Expression syntax.
//! Uses nom's parser combinators to define the grammar.

use std::str::from_utf8;

use nom::{self, alpha, alphanumeric, multispace, IResult};

use super::ast::*;
use eval::{Eval, Function, Value};


// TODO(xion): switch from parsers expecting &[u8] to accepting &str;
// this will get rid of the hack in float_literal() and possibly other cruft


// Grammar utilities.

/// Make the underlying parser assume UTF8-encoded input
/// and output String objects.
macro_rules! string (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i, map_res!($submac!($($args)*), from_utf8), String::from);
    );
    ($i:expr, $f:expr) => (
        string!($i, call!($f));
    );
);

/// Make the underlying parser optional,
/// but unlike opt! it is treating incomplete input as parse error.
macro_rules! maybe (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        opt!($i, complete!($submac!($($args)*)));
    );
    ($i:expr, $f:expr) => (
        maybe!($i, call!($f));
    );
);

/// Parse a sequence that matches the first parser followed by the second parser.
/// Return consumed input as the result (like recognize! does).
macro_rules! seq (
    // TODO(xion): generalize to arbitrary number of arguments (using chain!())
    ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => ({
        // Unfortunately, this cannot be implemented straightforwardly as:
        //     recognize!($i, pair!($submac!($($args)*), $submac2!($($args2)*)));
        // because Rust compiler fails to carry out the type inference correctly
        // in the generated code.
        //
        // Below is therefore essentially a rewrite of nom's recognize!() macro.
        use nom::HexDisplay;
        match pair!($i, $submac!($($args)*), $submac2!($($args2)*)) {
            IResult::Error(a)      => IResult::Error(a),
            IResult::Incomplete(i) => IResult::Incomplete(i),
            IResult::Done(i, _) => {
                let index = ($i).offset(i);
                IResult::Done(i, &($i)[..index])
            },
        }
    });
    ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
        seq!($i, $submac!($($args)*), call!($g));
    );
    ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
        seq!($i, call!($f), $submac!($($args)*));
    );
    ($i:expr, $f:expr, $g:expr) => (
        seq!($i, call!($f), call!($g));
    );
);

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

/// Matches exactly one character from the specified string.
/// This is like one_of!, but returns the matched char as &[u8] (assumming UTF8).
macro_rules! char_of (
    ($i:expr, $inp:expr) => (
        map!($i, one_of!($inp), |c: char| &$i[0..c.len_utf8()]);
    );
);


// Grammar constants.

const FUNCTIONAL_BINARY_OPS: &'static str = "&$";
const ADDITIVE_BINARY_OPS: &'static str = "+-";
const MULTIPLICATIVE_BINARY_OPS: &'static str = "*/%";
const POWER_OP: &'static str = "**";
const UNARY_OPS: &'static str = "+-!";

const RESERVED_WORDS: &'static [&'static str] = &[
    "const", "do", "else", "false", "for", "if", "let", "true", "while",
];

const DIGITS: &'static str = "0123456789";
const FLOAT_REGEX: &'static str = r"(0|[1-9][0-9]*)\.[0-9]+(e[+-]?[1-9][0-9]*)?";
const ESCAPE: &'static str = "\\";

const UNDERSCORE_SUFFIXES: &'static str = "bifs";


// Grammar definition.

/// Root symbol of the grammar.
named!(pub expression( &[u8] ) -> Box<Eval>, chain!(e: functional, || { e }));

/// functional ::== joint [FUNCTIONAL_OP joint]*
named!(functional( &[u8] ) -> Box<Eval>, chain!(
    first: joint ~
    rest: many0!(pair!(
        string!(multispaced!(char_of!(FUNCTIONAL_BINARY_OPS))),
        joint
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// joint ::== conditional | lambda
named!(joint( &[u8] ) -> Box<Eval>, alt!(conditional | lambda));

/// lambda ::== '|' ARGS '|' lambda
named!(lambda( &[u8] ) -> Box<Eval>, chain!(
    multispaced!(tag!("|")) ~
    args: separated_list!(multispaced!(tag!(",")), identifier) ~
    multispaced!(tag!("|")) ~
    body: joint,
    move || {
        Box::new(ScalarNode{
            value: Value::from(Function::from_lambda(args, body))
        }) as Box<Eval>
    }
));

/// conditional ::== comparison ['?' comparison ':' conditional]
named!(conditional( &[u8] ) -> Box<Eval>, map!(
    pair!(comparison, maybe!(chain!(
        multispaced!(tag!("?")) ~
        then: comparison ~
        multispaced!(tag!(":")) ~
        else_: conditional,
        move || (then, else_)
    ))),
    |(cond, maybe_then_else)| {
        match maybe_then_else {
            None => cond,
            Some((then, else_)) => Box::new(
                ConditionalNode{cond: cond, then: then, else_: else_}
            ) as Box<Eval>,
        }
    }
));

/// comparison ::== argument [COMPARISON_OP argument]
named!(comparison( &[u8] ) -> Box<Eval>, chain!(
    // TODO(xion): consider supporting chained comparisons a'la Python
    left: argument ~
    maybe_right: maybe!(pair!(
        string!(multispaced!(alt!(
            tag!("<=") | tag!(">=") | tag!("==") | tag!("!=") | char_of!("<>@")
        ))),
        argument
    )),
    move || {
        match maybe_right {
            None => left,
            Some(right) => Box::new(
                BinaryOpNode::new(Associativity::Left, left, vec![right])
            ) as Box<Eval>,
        }
    }
));

/// argument ::== term (ADDITIVE_BIN_OP term)*
named!(argument( &[u8] ) -> Box<Eval>, chain!(
    first: term ~
    rest: many0!(pair!(
        string!(multispaced!(char_of!(ADDITIVE_BINARY_OPS))),
        term
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// term ::== factor (MULTIPLICATIVE_BIN_OP factor)*
named!(term( &[u8] ) -> Box<Eval>, chain!(
    first: factor ~
    rest: many0!(pair!(
        string!(multispaced!(char_of!(MULTIPLICATIVE_BINARY_OPS))),
        factor
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// factor ::== power (POWER_OP power)*
named!(factor( &[u8] ) -> Box<Eval>, chain!(
    first: power ~
    rest: many0!(pair!(
        string!(multispaced!(tag!(POWER_OP))),
        power
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// power ::== [UNARY_OP] (function_call | atom) subscript*
named!(power( &[u8] ) -> Box<Eval>, chain!(
    ops: many0!(string!(multispaced!(char_of!(UNARY_OPS)))) ~
    power: atom ~
    trailers: many0!(trailer),
    move || {
        let mut result = power;

        // trailers (subscripts & function calls) have higher priority
        // than any unary operators, so we build their AST node(s) first
        for trailer in trailers {
            result = match trailer {
                Trailer::Subscript(index) =>
                    Box::new(SubscriptNode{object: result, index: index}),
                Trailer::Args(args) =>
                    Box::new(FunctionCallNode{func: result, args: args}),
            };
        }

        // then, we build nodes for any unary operators that may have been
        // prepended to the whole thing (in reverse order,
        // so that `---foo` means `-(-(-foo))`)
        for op in ops.into_iter().rev() {
            result = Box::new(UnaryOpNode{op: op, arg: result});
        }

        result
    }
));

/// trailer ::== '[' expression ']' | '(' ARGS ')'
enum Trailer { Subscript(Box<Eval>), Args(Vec<Option<Box<Eval>>>) }
named!(trailer( &[u8] ) -> Trailer, alt!(
    delimited!(multispaced!(tag!("[")),
               expression,
               multispaced!(tag!("]"))) => { |s| Trailer::Subscript(s) }
    |
    delimited!(multispaced!(tag!("(")),
               separated_list!(multispaced!(tag!(",")), map!(expression, Some)),
               multispaced!(tag!(")"))) => { |args| Trailer::Args(args) }
));

/// atom ::== OBJECT | ARRAY | BOOLEAN | SYMBOL | FLOAT | INTEGER | STRING | '(' expression ')'
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    object_value | array_value |
    bool_value | symbol_value | float_value | int_value | string_value |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

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
    |attrs| { Box::new(ObjectNode{attributes: attrs}) }
));

/// ARRAY ::== '[' [expression] (',' expression)* ']'
named!(array_value( &[u8] ) -> Box<Eval>, map!(
    delimited!(
        multispaced!(tag!("[")),
        separated_list!(multispaced!(tag!(",")), expression),
        multispaced!(tag!("]"))
    ),
    |items| { Box::new(ArrayNode{elements: items}) }
));

named!(bool_value( &[u8] ) -> Box<Eval>, alt!(
    tag!("false") => { |_| Box::new(ScalarNode{value: Value::from(false)}) } |
    tag!("true") => { |_| Box::new(ScalarNode{value: Value::from(true)}) }
));

named!(symbol_value( &[u8] ) -> Box<Eval>, map!(identifier, |value: String| {
    Box::new(ScalarNode{value: Value::Symbol(value)})
}));
named!(identifier( &[u8] ) -> String, alt!(
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

named!(int_value( &[u8] ) -> Box<Eval>, map_res!(int_literal, |value: String| {
    value.parse::<i64>().map(|i| Box::new(ScalarNode{value: Value::from(i)}))
}));
named!(int_literal( &[u8] ) -> String, string!(alt!(
    seq!(char_of!(&DIGITS[1..]), many0!(char_of!(DIGITS))) | tag!("0")
)));

named!(float_value( &[u8] ) -> Box<Eval>, map_res!(float_literal, |value: String| {
    value.parse::<f64>().map(|f| Box::new(ScalarNode{value: Value::from(f)}))
}));
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
        _ => panic!("unexpected IResult from re_find!"),
    }
}

named!(string_value( &[u8] ) -> Box<Eval>, map!(string_literal, |value: String| {
    Box::new(ScalarNode{value: Value::String(value)})
}));
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
