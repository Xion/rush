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
fn constant_boolean() {
    assert_noop_eval("true");
    assert_noop_eval("false");
}

#[test]
fn constant_array_empty() {
    const EXPR: &'static str = "[]";
    let expected = "";
    assert_eq!(expected, eval(EXPR));
}

#[test]
fn constant_array_1element() {
    const ELEMENT: &'static str = "foo";
    let expr = format!("[{}]", ELEMENT);
    assert_eq!(ELEMENT, eval(&expr));
}

#[test]
fn constant_array_integers() {
    const ELEMENTS: &'static [i64] = &[13, 42, 100, 256];
    let expr = format!("[{}]", join(ELEMENTS, ","));
    let actual: Vec<_> = eval(&expr)
        .split('\n').map(|s| s.parse::<i64>().unwrap()).collect();
    assert_eq!(ELEMENTS, &actual[..]);
}

#[test]
fn constant_array_floats() {
    const ELEMENTS: &'static [f64] = &[-13.5, 0.00002, 42.007, 999999999.7];
    let expr = format!("[{}]", join(ELEMENTS, ","));
    let actual: Vec<_> = eval(&expr)
        .split('\n').map(|s| s.parse::<f64>().unwrap()).collect();
    assert_eq!(ELEMENTS, &actual[..]);
}

#[test]
fn constant_array_strings() {
    const ELEMENTS: &'static [&'static str] = &["foo", "bar", "baz"];
    let expr = format!("[{}]", join(ELEMENTS, ","));
    let actual: Vec<_> = eval(&expr).split('\n').map(str::to_string).collect();
    assert_eq!(ELEMENTS, &actual[..]);
}

#[test]
fn constant_array_quoted_strings() {
    const ELEMENTS: &'static [&'static str] = &["Alice", "has", "a", "cat"];
    let expr = format!("[{}]", ELEMENTS.iter()
        .map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(","));
    let actual: Vec<_> = eval(&expr).split('\n').map(str::to_string).collect();
    assert_eq!(ELEMENTS, &actual[..]);
}

#[test]
fn constant_array_booleans() {
    const ELEMENTS: &'static [bool] = &[true, false, false, true, true];
    let expr = format!("[{}]", join(ELEMENTS, ","));
    let actual: Vec<_> = eval(&expr)
        .split('\n').map(|s| s.parse::<bool>().unwrap()).collect();
    assert_eq!(ELEMENTS, &actual[..]);
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
fn identity_on_boolean() {
    assert_noop_apply("_", "true");
    assert_noop_apply("_", "false");
}

#[test]
fn input_conversion_integer() {
    assert_noop_apply("_i", "42");
    assert_eq!(empty(), apply("_i", "42.42"));
    assert_eq!(empty(), apply("_i", "true"));
    assert_eq!(empty(), apply("_i", "foo"));
}

#[test]
fn input_conversion_float() {
    assert_noop_apply("_f", "42.42");
    assert_eq!("42.0", apply("_f", "42"));
    assert_eq!(empty(), apply("_f", "true"));
    assert_eq!(empty(), apply("_f", "foo"));
}

#[test]
fn input_conversion_boolean() {
    assert_noop_apply("_b", "true");
    assert_noop_apply("_b", "false");
    assert_eq!(empty(), apply("_b", "42"));
    assert_eq!(empty(), apply("_b", "42.42"));
    assert_eq!(empty(), apply("_b", "foo"));
}

#[test]
fn input_conversion_string() {
    assert_noop_apply("_s", "42");
    assert_noop_apply("_s", "42.42");
    assert_noop_apply("_s", "true");
    assert_noop_apply("_s", "foo");
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
fn unary_plus_string() {
    assert_apply_error("+_", "foo");
}

#[test]
fn unary_plus_boolean() {
    assert_apply_error("+_", "true");
    assert_apply_error("+_", "false");
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

// TODO(xion): tests for binary minus, multiplication, division, string formatting

#[test]
fn subscript_of_array_constant() {
    assert_eq!("42", eval("[42][0]"));
    assert_eq!("42", eval("[13, 42][1]"));
    assert_eq!("42", eval("[[42]][0][0]"));
    assert_eval_error("[][0]");
    assert_eval_error("[42][1]");
}

#[test]
fn subscript_of_array_input() {
    const INPUT: &'static [&'static str] = &["foo", "bar"];
    assert_eq!("foo", reduce("_[0]", INPUT));
    assert_eq!("bar", reduce("_[1]", INPUT));
    assert_eq!("foo", reduce("[_][0][0]", INPUT));
    assert_eq!("other", reduce("[_, [other]][1][0]", INPUT));
    assert_reduce_error("_[42]", INPUT);
}


// Utility functions.

fn join<T: ToString>(array: &[T], sep: &str) -> String {
    array.iter().map(T::to_string).collect::<Vec<_>>().join(sep)
}


// Assertions.
// TODO(xion): allow for more fine grained error assertions

fn assert_noop_eval(expr: &str) {
    assert_eq!(expr, eval(expr));
}

fn assert_noop_apply(expr: &str, input: &str) {
    assert_eq!(input, apply(expr, input));
}

fn assert_eval_error(expr: &str) {
    assert!(eval_ex(expr).is_err());
}

fn assert_apply_error(expr: &str, input: &str) {
    assert!(apply_ex(expr, input).is_err());
}

fn assert_reduce_error<'a>(expr: &str, input: &'a [&'a str]) {
    assert!(reduce_ex(expr, input).is_err());
}


// Wrappers around tested code.

/// Evaluate the expression without any input.
fn eval(expr: &str) -> String {
    match eval_ex(expr) {
        Ok(output) => output,
        Err(err) => { panic!("eval() error: {}", err); }
    }
}

fn eval_ex(expr: &str) -> Result<String, io::Error> {
    apply_ex(expr, "unused")
}

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

/// Applies an expression to input given as slice of strings.
/// This input is interpreted as an array by the given expression.
fn reduce<'a>(expr: &str, input: &'a [&'a str]) -> String {
    match reduce_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("reduce() error: {}", err); }
    }
}

fn reduce_ex<'a>(expr: &str, input: &'a [&'a str]) -> Result<String, io::Error> {
    let input = input.join("\n").to_string();

    let mut output: Vec<u8> = Vec::new();
    try!(ap::reduce(expr, input.as_bytes(), &mut output));

    // if the result turns out to be just a single line,
    // remove the trailing \n
    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_string();
    if result.chars().filter(|c| *c == '\n').count() == 1 {
        result.pop();
    }
    Ok(result)
}

/// Return the string representation of Value::Empty.
fn empty() -> String {
    format!("{}", ap::Value::Empty)
}
