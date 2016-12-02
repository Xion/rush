//! Module defining grammar symbols that form the main structure of the syntax.

use eval::{Eval, Function};
use parse::ast::*;
use super::literals::{atom, identifier};
use super::ops::*;


/// Root symbol of the grammar.
named!(pub expression( &[u8] ) -> Box<Eval>, chain!(e: assignment, || { e }));


/// Macros shortening the repetitive parts of defining syntactical constructs
/// involving binary operators.
macro_rules! binary (
    ($rule:ident($assoc:ident) => $first:ident ($op:ident $rest:ident)*) => (
        named!($rule( &[u8] ) -> Box<Eval>, chain!(
            first: $first ~
            rest: many0!(pair!($op, $rest)),
            move || {
                if rest.is_empty() { first }
                else { Box::new(
                    BinaryOpNode::new(Associativity::$assoc, first, rest)
                ) as Box<Eval> }
            }
        ));
    );
);
macro_rules! left_assoc (
    ($rule:ident => $first:ident ($op:ident $rest:ident)*) => (
        binary!($rule(Left) => $first ($op $rest)*);
    );
);
macro_rules! right_assoc (
    ($rule:ident => $first:ident ($op:ident $rest:ident)*) => (
        binary!($rule(Right) => $first ($op $rest)*);
    );
);


/// assignment ::== functional (ASSIGNMENT_OP functional)*
right_assoc!(assignment => functional (assignment_op functional)*);

/// functional ::== joint (FUNCTIONAL_OP joint)*
left_assoc!(functional => joint (functional_op joint)*);

/// joint ::== conditional | lambda | curried_op
named!(joint( &[u8] ) -> Box<Eval>, alt!(conditional | lambda | curried_op));

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

/// logical ::== comparison (LOGICAL_OP comparison)*
left_assoc!(logical => comparison (logical_op comparison)*);

/// comparison ::== argument [COMPARISON_OP argument]
named!(comparison( &[u8] ) -> Box<Eval>, chain!(
    // TODO(xion): consider supporting chained comparisons a'la Python
    // (we could use the left_assoc! macro then)
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
left_assoc!(argument => term (additive_op term)*);

/// term ::== factor (MULTIPLICATIVE_BIN_OP factor)*
left_assoc!(term => factor (multiplicative_op factor)*);

/// factor ::== power (POWER_OP power)*
left_assoc!(factor => power (power_op power)*);

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
