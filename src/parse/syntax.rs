//! Expression syntax.
//! Uses nom's parser combinators to define the grammar.

use std::str::from_utf8;

use nom::{self, alpha, alphanumeric, multispace, IResult};

use super::ast::*;
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

/// Parse a sequence that matches the first parser followed by the second parser.
/// Return consumed input as the result (like recognize! does).
macro_rules! seq(
    ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => ({
        use nom::HexDisplay;
        match $submac!($i, $($args)*) {
            IResult::Error(a)      => IResult::Error(a),
            IResult::Incomplete(i) => IResult::Incomplete(i),
            IResult::Done(i1,_)   => {
                match $submac2!(i1, $($args2)*) {
                    IResult::Error(a)      => IResult::Error(a),
                    IResult::Incomplete(i) => IResult::Incomplete(i),
                    IResult::Done(i2,_)   => {
                        if i2.is_empty() {
                            IResult::Done(i2, $i)
                        } else {
                            let index = ($i).offset(i2);
                            IResult::Done(i2, &($i)[..index])
                        }
                    }
                }
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
// TODO(xion): once recognize! is fixed upstream to properly handle empty
// residual input, use it recognize!(pair!(...)) in place of the above macro
// (relevant pull request: https://github.com/Geal/nom/pull/213)

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
macro_rules! char_of {
    ($i:expr, $inp:expr) => ({
        // For some inexplicable reasons, implementing this straightforwardly
        // as a macro doesn't provide enough type information in certain cases
        // (e.g. string!(multispaced!(char_of!(UNARY_OPS)))).
        // However, defining an auxiliary function and just calling it
        // seems to do the trick.

        #[inline(always)]
        fn _char_of(ops: &'static str) -> Box<Fn(&[u8]) -> IResult<&[u8], &[u8]>> {
            Box::new(move |input| {
                let (rest, c) = try_parse!(input, one_of!(ops));
                IResult::Done(rest, &input[0..c.len_utf8()])
            })
        }
        _char_of($inp)($i)
    });
}


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
        string!(multispaced!(char_of!(ADDITIVE_BINARY_OPS))),
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
        string!(multispaced!(char_of!(MULTIPLICATIVE_BINARY_OPS))),
        factor
    )),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode{first: first, rest: rest}
        ) as Box<Eval> }
    }
));

/// factor ::== [UNARY_OP] (function_call | atom) subscript*
named!(factor( &[u8] ) -> Box<Eval>, chain!(
    mut ops: many0!(string!(multispaced!(char_of!(UNARY_OPS)))) ~
    mut factor: alt!(
        // complete!(...) is necessary because `atom` can be a prefix
        // of `function_call`. Otherwise, trying to parse an atom
        // as function call will result in incomplete input
        // (because the pair of parentheses is "missing").
        // Using complete! forces the parses to interpret this IResult::Incomplete
        // as error (and thus try the `atom` branch) rather than bubble it up.
        complete!(function_call) | atom
    ) ~
    // TODO(xion): when functions are first-class values, we'll need to roll
    // parenthesized arguments from `function_call` into here
    subscripts: many0!(
        delimited!(multispaced!(tag!("[")), argument, multispaced!(tag!("]")))
    ),
    move || {
        // subscripting has higher priority than any possible unary operators,
        // so we build the AST node(s) for that first
        for subscript in subscripts {
            factor = Box::new(
                SubscriptNode{object: factor, index: subscript}
            ) as Box<Eval>
        }
        // then, we build nodes for any unary operators that may have been
        // prepended to the factor (in reverse order, so that `---foo` means
        // `-(-(-foo))`)
        ops.reverse();
        for op in ops.drain(..) {
            factor = Box::new(UnaryOpNode{op: op, arg: factor}) as Box<Eval>
        }
        factor
    }
));

/// function_call ::== identifier '(' ARGS ')'
named!(function_call( &[u8] ) -> Box<Eval>, chain!(
    name: identifier ~
    args: delimited!(multispaced!(tag!("(")), items, multispaced!(tag!(")"))),
    || { Box::new(FunctionCallNode{name: name, args: args}) as Box<Eval> }
));

/// items ::== expression (',' expression)*
named!(items( &[u8] ) -> Vec<Box<Eval>>,
       separated_list!(multispaced!(tag!(",")), argument));

/// atom ::== ARRAY | BOOLEAN | SYMBOL | FLOAT | INTEGER | STRING | '(' expression ')'
named!(atom( &[u8] ) -> Box<Eval>, alt!(
    array_value | bool_value | symbol_value | float_value | int_value | string_value |
    delimited!(multispaced!(tag!("(")), expression, multispaced!(tag!(")")))
));

named!(array_value( &[u8] ) -> Box<Eval>, map!(
    delimited!(multispaced!(tag!("[")), items, multispaced!(tag!("]"))),
    |items| { Box::new(ArrayNode{elements: items}) as Box<Eval> }
));

named!(bool_value( &[u8] ) -> Box<Eval>, alt!(
    tag!("false") => { |_| Box::new(ScalarNode{value: Value::Boolean(false)}) } |
    tag!("true") => { |_| Box::new(ScalarNode{value: Value::Boolean(true)}) }
));

named!(symbol_value( &[u8] ) -> Box<Eval>, map!(identifier, |value: String| {
    Box::new(ScalarNode{value: Value::Symbol(value)})
}));
named!(identifier( &[u8] ) -> String, alt!(
    // TODO(xion): we should use char_of! instead of is_a! here
    // to prohibit nonsensical suffixes longer than one char,
    // but it inexplicably fails; investigate
    string!(seq!(tag!("_"), opt!(is_a!(UNDERSCORE_SUFFIXES)))) |
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
    value.parse::<i64>().map(|i| Box::new(ScalarNode{value: Value::Integer(i)}))
}));
named!(int_literal( &[u8] ) -> String, string!(alt!(
    seq!(is_a!(&DIGITS[1..]), many0!(is_a!(DIGITS))) | tag!("0")
)));

named!(float_value( &[u8] ) -> Box<Eval>, map_res!(float_literal, |value: String| {
    value.parse::<f64>().map(|f| Box::new(ScalarNode{value: Value::Float(f)}))
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
    Box::new(ScalarNode{value: Value::String(value)})
}));
named!(string_literal( &[u8] ) -> String, string!(
    preceded!(tag!("\""), take_until_and_consume!("\""))
));
