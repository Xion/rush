//! Tests for unary operators.

use util::*;


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
