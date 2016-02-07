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
    assert_noop_apply("++_", "42");
    assert_noop_apply("+++_", "42");
}

#[test]
fn unary_plus_float() {
    assert_noop_apply("+_", "42.42");
    assert_noop_apply("++_", "42.42");
    assert_noop_apply("+++_", "42.42");
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
    let negated = format!("-{}", INPUT);
    assert_eq!(negated, apply("-_", INPUT));
    assert_eq!(INPUT, apply("--_", INPUT));
    assert_eq!(negated, apply("---_", INPUT));
}

#[test]
fn unary_minus_float() {
    const INPUT: &'static str = "42.42";
    let negated = format!("-{}", INPUT);
    assert_eq!(negated, apply("-_", INPUT));
    assert_eq!(INPUT, apply("--_", INPUT));
    assert_eq!(negated, apply("---_", INPUT));
}

#[test]
fn unary_bang_constant() {
    assert_eq!("false", eval("!true"));
    assert_eq!("true", eval("!!true"));
    assert_eq!("false", eval("!!!true"));
    assert_eq!("true", eval("!false"));
    assert_eq!("false", eval("!!false"));
    assert_eq!("true", eval("!!!false"));
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
    assert_eq!("4.0", eval("2.0 + 2.0"));
    assert_eq!("42.0", eval("-2.5 + 44.5"));
}

#[test]
fn binary_plus_constant_strings() {
    assert_eq!("foo", eval("\"\" + foo"));
    assert_eq!("foobar", eval("foo + bar"));
    assert_eq!("barbaz", eval("bar + \"baz\""));
}

#[test]
fn binary_plus_input_integers() {
    assert_noop_apply("_ + 0", "42");
    assert_noop_apply("0 + _", "42");
    assert_eq!("42", apply("_ + 40", "2"));
    assert_eq!("42", apply("40 + _", "2"));
    assert_eq!("6", apply("_ + _", "3"));
    assert_eq!("12", apply("_ + _ + _", "4"));
}
// TODO(xion): binary_plus_input_floats
// TODO(xion): binary_plus_inpit_strings

#[test]
fn binary_minus_constant_integers() {
    assert_eq!("0", eval("0 - 0"));
    assert_eq!("2", eval("2 - 0"));
    assert_eq!("3", eval("5 - 2"));
    assert_eq!("-4", eval("1 - 5"));
    assert_eq!("-2", eval("-1 - 1"));
    assert_eq!("1", eval("-3 - -4"));
}

#[test]
fn binary_minus_constant_floats() {
    assert_eq!("0.0", eval("0.0 - 0.0"));
    assert_eq!("2.0", eval("2.0 - 0.0"));
    assert_eq!("3.0", eval("5.0 - 2.0"));
    assert_eq!("-4.0", eval("1.0 - 5.0"));
    assert_eq!("-2.0", eval("-1.0 - 1.0"));
    assert_eq!("1.0", eval("-3.0 - -4.0"));
}

#[test]
fn binary_minus_input_integers() {
    assert_noop_apply("_ - 0", "42");
    assert_eq!("-42", apply("0 - _", "42"));
    assert_eq!("40", apply("42 - _", "2"));
    assert_eq!("-2", apply("40 - _", "42"));
    assert_eq!("0", apply("_ - _", "42"));
    assert_eq!("-42", apply("_ - _ - _", "42"));
    assert_noop_apply("_ - (_ - _)", "42");
}
// TODO(xion): binary_minus_input_floats

#[test]
fn multiplication_constant_integers() {
    assert_eq!("0", eval("0 * 0"));
    assert_eq!("0", eval("2 * 0"));
    assert_eq!("3", eval("3 * 1"));
    assert_eq!("-4", eval("4 * -1"));
    assert_eq!("2", eval("-2 * -1"));
}

#[test]
fn multiplication_constant_floats() {
    assert_eq!("0.0", eval("0.0 * 0.0"));
    assert_eq!("0.0", eval("2.0 * 0.0"));
    assert_eq!("3.0", eval("3.0 * 1.0"));
    assert_eq!("-4.0", eval("4.0 * -1.0"));
    assert_eq!("2.0", eval("-2.0 * -1.0"));
}

// TODO(xion): tests for division, string formatting

#[test]
fn subscript_of_array_constant() {
    assert_eq!("42", eval("[42][0]"));
    assert_eq!("42", eval("[13, 42][1]"));
    assert_eq!("42", eval("[[42]][0][0]"));
    assert_eq!("c", eval("[a, b, c][-1]"));
    assert_eval_error("[][0]");
    assert_eval_error("[42][1]");
    assert_eval_error("[42][-2]");
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

#[test]
fn subscript_on_string_constant() {
    assert_eq!("f", eval("foo[0]"));
    assert_eq!("a", eval("\"bar\"[1]"));
    assert_eval_error("\"\"[]");
    assert_eval_error("baz[42]");
}

#[test]
fn subscript_on_string_input() {
    const INPUT: &'static str = "hello";
    assert_eq!("h", apply("_[0]", INPUT));
    assert_eq!("l", apply("_[2]", INPUT));
    assert_eq!("o", apply("_[-1]", INPUT));
    assert_eq!("e", apply("_[-4]", INPUT));
    assert_apply_error("_[42]", INPUT);
    assert_apply_error("_[-42]", INPUT);
}

#[test]
fn function_call_1arg_constant() {
    assert_eq!("42", eval("abs(42)"));
    assert_eq!("5", eval("len(hello)"));
}

#[test]
fn function_call_1arg_input() {
    assert_noop_apply("abs(_)", "42");
    assert_eq!("5", apply("len(_)", "hello"));
}

#[test]
fn function_call_2args_constant() {
    assert_eq!("he\n\no", eval("split(hello, l)"));
}

#[test]
fn function_call_2args_input() {
    assert_eq!("he\n\no", apply("split(_, l)", "hello"));
}

#[test]
fn function_call_3args_constant() {
    assert_eq!("pot", eval("sub(i, o, pit)"));
    assert_eq!("", eval("sub(a, \"\", aaa)"));
}

#[test]
fn function_call_3args_input() {
    assert_eq!("pot", apply("sub(i, o, _)", "pit"));
    assert_eq!("", apply("sub(a, \"\", _)", "aaa"));
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
