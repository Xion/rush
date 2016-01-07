//! Test crate.

extern crate ap;

use std::str::from_utf8;


#[test]
fn constant_integer() {
    const EXPR: &'static str = "42";
    assert_eq!(EXPR, apply(EXPR, "unused"));
}

#[test]
fn constant_float() {
    const EXPR: &'static str = "42.42";
    assert_eq!(EXPR, apply(EXPR, "unused"));
}

#[test]
fn constant_float_scientific() {
    const EXPR: &'static str = "42.4e2";
    let expected = EXPR.parse::<f64>().unwrap().to_string();
    assert_eq!(expected, apply(EXPR, "unused"));
}

#[test]
fn constant_string() {
    const EXPR: &'static str = "foo";
    assert_eq!(EXPR, apply(EXPR, "unused"));
}

#[test]
fn constant_quoted_string() {
    const STRING: &'static str = "foo";
    let expr = &format!("\"{}\"", STRING);
    assert_eq!(STRING, apply(expr, "unused"));
}

#[test]
fn identity() {
    const INPUT: &'static str = "42";
    assert_eq!(INPUT, apply("_", INPUT));
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
