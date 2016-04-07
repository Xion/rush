//! Module defining grammar symbols that form the main structure of the syntax.

use eval::{Eval, Function};
use parse::ast::*;
use super::literals::{atom, identifier};
use super::ops::*;


/// Root symbol of the grammar.
named!(pub expression( &[u8] ) -> Box<Eval>, chain!(e: functional, || { e }));


/// functional ::== joint (FUNCTIONAL_OP joint)*
named!(functional( &[u8] ) -> Box<Eval>, chain!(
    first: joint ~
    rest: many0!(pair!(functional_op, joint)),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// joint ::== conditional | lambda | curried_op
named!(joint( &[u8] ) -> Box<Eval>, alt!(conditional | lambda | curried_op));

/// conditional ::== logical ['?' logical ':' conditional]
named!(conditional( &[u8] ) -> Box<Eval>, map!(
    pair!(logical, maybe!(chain!(
        multispaced!(tag!("?")) ~
        then: logical ~
        multispaced!(tag!(":")) ~
        else_: conditional,
        move || (then, else_)
    ))),
    |(cond, maybe_then_else)| {
        match maybe_then_else {
            None => cond,
            Some((then, else_)) => Box::new(
                ConditionalNode::new(cond, then, else_)
            ) as Box<Eval>,
        }
    }
));

/// lambda ::== '|' ARGS '|' joint
named!(lambda( &[u8] ) -> Box<Eval>, chain!(
    multispaced!(tag!("|")) ~
    args: separated_list!(multispaced!(tag!(",")), identifier) ~
    multispaced!(tag!("|")) ~
    body: joint,
    move || { Box::new(
        ScalarNode::from(Function::from_lambda(args, body))
    ) as Box<Eval> }
));

/// curried_op ::== '(' (atom BINARY_OP) | (BINARY_OP atom) | BINARY_OP ')'
named!(curried_op( &[u8] ) -> Box<Eval>, delimited!(
    multispaced!(tag!("(")),
        alt!(
            pair!(atom, binary_op) => { |(arg, op)| Box::new(
                CurriedBinaryOpNode::with_left(op, arg)
            ) as Box<Eval> }
            |
            pair!(binary_op, atom) => { |(op, arg)| Box::new(
                CurriedBinaryOpNode::with_right(op, arg)
            ) as Box<Eval> }
            |
            binary_op => { |op| Box::new(
                CurriedBinaryOpNode::with_none(op)
            ) as Box<Eval> }
        ),
    multispaced!(tag!(")"))
));

/// logical ::== comparison (LOGICAL_OP comparison)*
named!(logical( &[u8] ) -> Box<Eval>, chain!(
    first: comparison ~
    rest: many0!(pair!(logical_op, comparison)),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// comparison ::== argument [COMPARISON_OP argument]
named!(comparison( &[u8] ) -> Box<Eval>, chain!(
    // TODO(xion): consider supporting chained comparisons a'la Python
    left: argument ~
    maybe_right: maybe!(pair!(comparison_op, argument)),
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
    rest: many0!(pair!(additive_op, term)),
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
    rest: many0!(pair!(multiplicative_op, factor)),
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
    rest: many0!(pair!(power_op, power)),
    move || {
        if rest.is_empty() { first }
        else { Box::new(
            BinaryOpNode::new(Associativity::Left, first, rest)
        ) as Box<Eval> }
    }
));

/// power ::== UNARY_OP* atom trailer*
named!(power( &[u8] ) -> Box<Eval>, chain!(
    ops: many0!(unary_op) ~
    power: atom ~
    trailers: many0!(trailer),
    move || {
        let mut result = power;

        // trailers (subscripts & function calls) have higher priority
        // than any unary operators, so we build their AST node(s) first
        for trailer in trailers {
            result = match trailer {
                Trailer::Subscript(index) =>
                    Box::new(SubscriptNode::new(result, index)),
                Trailer::Args(args) =>
                    Box::new(FunctionCallNode::new(result, args)),
            };
        }

        // then, we build nodes for any unary operators that may have been
        // prepended to the whole thing (in reverse order,
        // so that `---foo` means `-(-(-foo))`)
        for op in ops.into_iter().rev() {
            result = Box::new(UnaryOpNode::new(op, result));
        }

        result
    }
));

/// trailer ::== '[' INDEX ']' | '(' ARGS ')'
enum Trailer { Subscript(Index), Args(Vec<Box<Eval>>) }
named!(trailer( &[u8] ) -> Trailer, alt!(
    delimited!(multispaced!(tag!("[")),
               index,
               multispaced!(tag!("]"))) => { |idx| Trailer::Subscript(idx) }
    |
    delimited!(multispaced!(tag!("(")),
               separated_list!(multispaced!(tag!(",")), expression),
               multispaced!(tag!(")"))) => { |args| Trailer::Args(args) }
));
named!(index( &[u8] ) -> Index, alt!(
    chain!(
        left: maybe!(expression) ~
        multispaced!(tag!(":")) ~
        right: maybe!(expression),
        move || { Index::Range(left, right) }
    ) |
    expression => { |expr| Index::Point(expr) }
));
