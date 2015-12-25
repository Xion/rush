//! Parser code for the expression syntax.

use std::str::from_utf8;

use nom::{alphanumeric, multispace, IResult, Needed, Err, ErrorKind};

use ast::*;
use eval::Value;


named!(value<&[u8], ValueNode>, chain!(
    value: map_res!(alt!(tag!("_") | alphanumeric), from_utf8),
    || { value.parse::<ValueNode>().unwrap() }
));
named!(binary_op<&[u8], BinaryOpNode>, chain!(
    left: value ~
    multispace? ~
    op: map_res!(is_a!("+"), from_utf8) ~
    multispace? ~
    right: value,
    || { BinaryOpNode{op: op.to_string(),
                      left: Box::new(left),
                      right: Box::new(right)} }
));

fn expr(input: &[u8]) -> IResult<&[u8], Box<Eval>> {
    // TODO(xion): figure out how to do this with alt!() rather than manually
    // (the problem with alt! is that it uses `match` for branching
    // and that doesn't work since *Node results are unrelated types and cannot
    // be matched against)
    if let IResult::Done(input, output) = binary_op(input) {
        assert!(input.is_empty());
        return IResult::Done(input, Box::new(output) as Box<Eval>);
    }
    if let IResult::Done(input, output) = value(input) {
        assert!(input.is_empty());
        return IResult::Done(input, Box::new(output) as Box<Eval>);
    }

    // TODO(xion): introduce custom error type instead of the default numeric
    IResult::Error(Err::Code(ErrorKind::Custom(404)))
}


pub fn parse(input: &str) -> Box<Eval> {
    match expr(input.trim().as_bytes()) {
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
