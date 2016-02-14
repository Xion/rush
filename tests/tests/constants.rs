//! Tests for constant expressions.

use std::collections::HashMap;

use util::*;


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
fn constant_object_empty() {
    assert_noop_eval("{}");
}

#[test]
fn constant_object_1attribute() {
    assert_noop_eval("{\"a\":2}");
    assert_eval_error("{2: 3}");  // because key has to be string
}

#[test]
fn constant_object() {
    let mut elems = HashMap::new();
    {
        elems.insert("a".to_owned(), "foo".to_owned());
        elems.insert("b".to_owned(), "bar".to_owned());
    }
    let expr = format!("{{{}}}", elems.iter()
        .map(|(ref k, ref v)| format!("{}:{}", k, v))
        .collect::<Vec<_>>().join(","));
    let actual = parse_json_stringmap(&eval(&expr));
    assert_eq!(elems, actual);
}

// TODO(xion): more constant objects' tests
