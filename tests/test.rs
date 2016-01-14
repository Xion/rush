//! Test crate.

extern crate ap;

use std::io;
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
fn constant_float_zero() {
    assert_noop_eval("0.0");
}

#[test]
fn constant_float_fraction() {
    assert_noop_eval("0.42");
}

#[test]
fn constant_float_scientific() {
    const EXPR: &'static str = "42.4e2";
    let expected = EXPR.parse::<f64>().unwrap().to_string() + ".0";
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
fn identity_on_string() {
    assert_noop_apply("_", "foo");
}

#[test]
fn identity_on_int() {
    assert_noop_apply("_", "42");
}

#[test]
fn identity_on_float() {
    assert_noop_apply("_", "42.42");
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

#[test]
fn binary_plus_constant_integers() {
    assert_eq!("0", eval("0 + 0"));
    assert_eq!("2", eval("0 + 2"));
    assert_eq!("4", eval("2 + 2"));
    assert_eq!("42", eval("-2 + 44"));
}

#[test]
fn binary_plus_constant_floats() {
    assert_eq!("0.0", eval("0.0 + 0.0"));
    assert_eq!("2.0", eval("0 + 2.0"));
    assert_eq!("4.0", eval("2.0 + 2"));
    assert_eq!("42.0", eval("-2.5 + 44.5"));
}

#[test]
fn binary_plus_constant_strings() {
    assert_eq!("foo", eval("\"\" + foo"));
    assert_eq!("foobar", eval("foo + bar"));
    assert_eq!("barbaz", eval("bar + \"baz\""));
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
    match apply_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("apply() error: {}", err); }
    }
}

fn apply_ex(expr: &str, input: &str) -> Result<String, io::Error> {
    let mut extra_newline = false;
    let mut input = input.to_string();
    if !input.ends_with("\n") {
        input.push('\n');
        extra_newline = true;
    }

    let mut output: Vec<u8> = Vec::new();
    try!(ap::apply(expr, input.as_bytes(), &mut output));

    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_string();
    if extra_newline {
        result.pop();  // remove trailing \n
    }
    Ok(result)
}

/// Evaluate the expression without any input.
fn eval(expr: &str) -> String {
    apply(expr, "unused")
}
