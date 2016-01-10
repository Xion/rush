//! Test crate.

extern crate ap;

use std::str::from_utf8;


#[test]
fn constant_boolean_true() {
    assert_noop_eval("true");
}

#[test]
fn constant_boolean_false() {
    assert_noop_eval("true");
}

#[test]
fn constant_integer() {
    assert_noop_eval("42");
}

#[test]
fn constant_integer_negative() {
    // Note that this may actually be interpreted as unary minus expression,
    // but the user wouldn't care about that so we consider it constant.
    assert_noop_eval("-42");
}

#[test]
fn constant_float() {
    assert_noop_eval("42.42");
}

#[test]
fn constant_float_scientific() {
    const EXPR: &'static str = "42.4e2";
    let expected = EXPR.parse::<f64>().unwrap().to_string();
    assert_eq!(expected, eval(EXPR));
}

#[test]
fn constant_float_negative() {
    // Note that this may actually be interpreted as unary minus expression,
    // but the user wouldn't care about that so we consider it constant.
    assert_noop_eval("-42.42");
}

#[test]
fn constant_string() {
    assert_noop_eval("foo");
}

#[test]
fn constant_quoted_string() {
    const STRING: &'static str = "foo";
    let expr = &format!("\"{}\"", STRING);
    assert_eq!(STRING, eval(expr));
}

#[test]
fn identity() {
    const INPUT: &'static str = "42";
    assert_eq!(INPUT, apply("_", INPUT));
}

#[test]
fn unary_plus_integer() {
    assert_noop_apply("+_", "42");
}

#[test]
fn unary_plus_float() {
    assert_noop_apply("+_", "42.42");
}

#[test]
fn unary_minus_integer() {
    const INPUT: &'static str = "42";
    let expected = format!("-{}", INPUT);
    assert_eq!(expected, apply("-_", INPUT));
}

#[test]
fn unary_minus_float() {
    const INPUT: &'static str = "42.42";
    let expected = format!("-{}", INPUT);
    assert_eq!(expected, apply("-_", INPUT));
}

#[test]
fn unary_bang_constant() {
    assert_eq!("false", eval("!true"));
    assert_eq!("true", eval("!false"));
}

#[test]
fn unary_bang_input() {
    assert_eq!("false", apply("!_", "true"));
    assert_eq!("true", apply("!_", "false"));
}


// Assertions.

fn assert_noop_apply(expr: &str, input: &str) {
    assert_eq!(input, apply(expr, input));
}

fn assert_noop_eval(expr: &str) {
    assert_eq!(expr, eval(expr));
}


// Utility functions.

/// Applies an expression to input given as a string.
///
/// Single- and multiline strings are handled automatically:
/// if the input didn't end with a newline, output won't either.
fn apply(expr: &str, input: &str) -> String {
    let mut extra_newline = false;
    let mut input = input.to_string();
    if !input.ends_with("\n") {
        input.push('\n');
        extra_newline = true;
    }

    let mut output: Vec<u8> = Vec::new();
    if let Err(err) = ap::apply(expr, input.as_bytes(), &mut output) {
        panic!("apply() error: {}", err);
    }

    let mut result = from_utf8(&output).unwrap().to_string();
    if extra_newline {
        result.pop();  // remove trailing \n
    }
    result
}

/// Evaluate the expression without any input.
fn eval(expr: &str) -> String {
    apply(expr, "unused")
}
