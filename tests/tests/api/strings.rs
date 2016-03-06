//! Tests for the string-related API functions.

use util::*;


#[test]
fn rev() {
    assert_eq!("oof", apply("rev(_)", "foo"));
    assert_noop_apply("rev(_)", "racecar");
    assert_apply_error("rev(_)", "42");
    assert_apply_error("rev(_)", "13.42");
    assert_apply_error("rev(_)", "false");
    assert_eval_error(&format!("rev({})", "[]"));
    assert_eval_error(&format!("rev({})", "{}"));
}

#[test]
fn split_strings() {
    assert_eq!("", apply("split(X, _)", ""));
    assert_eq!("foo", apply("split(X, _)", "foo"));
    assert_eq!(unlines!("foo", "bar"), apply("split(X, _)", "fooXbar"));
    assert_eq!(unlines!("foo", ""), apply("split(X, _)", "fooX"));
    assert_eq!(unlines!("", "foo"), apply("split(X, _)", "Xfoo"));
    assert_eq!(unlines!("", ""), apply("split(X, _)", "X"));
}

#[test]
fn split_non_strings() {
    // ...
}

#[test]
fn join_() {
    // ...
}
