//! Tests for the string-related API functions.

use util::*;


#[test]
fn chr() {
    assert_eq!("A", apply("chr(_)", "65"));
    assert_eq!("a", apply("chr(_)", "97"));
    assert_apply_error("chr(_)", "a");
    assert_apply_error("chr(_)", "foo");
    assert_apply_error("chr(_)", "3.14");
    assert_apply_error("chr(_)", "-1");
    assert_eval_error("chr([])");
    assert_eval_error("chr({})");
}

#[test]
fn ord() {
    assert_eq!("65", apply("ord(_)", "A"));
    assert_eq!("97", apply("ord(_)", "a"));
    assert_apply_error("ord(_)", "foo");
    assert_apply_error("ord(_)", "42");
    assert_apply_error("ord(_)", "-12");
    assert_apply_error("ord(_)", "2.71");
    assert_eval_error("ord([])");
    assert_eval_error("ord({})");
}

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
    assert_apply_error("split(X, _)", "42");
    assert_apply_error("split(X, _)", "13.42");
    assert_apply_error("split(X, _)", "false");
    assert_eval_error(&format!("split(X, {})", "[]"));
    assert_eval_error(&format!("split(X, {})", "{}"));
}

#[test]
fn join_() {
    assert_eq!("", apply_lines("join(X, _)", &[""]));
    assert_eq!("foo", apply_lines("join(X, _)", &["foo"]));
    assert_eq!("fooXbar", apply_lines("join(X, _)", &["foo", "bar"]));
    assert_eq!("falseXtrue", apply_lines("join(X, _)", &[false, true]));
    assert_eval_error(&format!("join(X, {})", "false"));
    assert_eval_error(&format!("join(X, {})", "foo"));
    assert_eval_error(&format!("join(X, {})", "42"));
    assert_eval_error(&format!("join(X, {})", "13.42"));
    assert_eval_error(&format!("join(X, {})", "{}"));
}

// TODO(xion): tests for sub(), especially w/ regex and replacement function

#[test]
fn before_string() {
    assert_eq!("", apply("before(\"\", _)", ""));
    assert_eq!("", apply("before(bar, _)", ""));
    assert_eq!("", apply("before(bar, _)", "bar"));
    assert_eq!("foo", apply("before(bar, _)", "foobar"));
    assert_eq!("", apply("before(baz, _)", "foobar"));
    assert_apply_error("before(bar, _)", "42");
    assert_apply_error("before(bar, _)", "3.14");
    assert_eval_error("before(bar, [])");
    assert_eval_error("before(bar, {})");
}

#[test]
fn before_regex() {
    assert_eq!("", apply("before(//, _)", ""));
    assert_eq!("", apply("before(/bar/, _)", ""));
    assert_eq!("", apply("before(/bar/, _)", "bar"));
    assert_eq!("foo", apply("before(/bar/, _)", "foobar"));
    assert_eq!("", apply("before(/baz/, _)", "foobar"));
    assert_apply_error("before(/bar/, _)", "42");
    assert_apply_error("before(/bar/, _)", "3.14");
    assert_eval_error("before(/bar/, [])");
    assert_eval_error("before(/bar/, {})");
}

#[test]
fn after_string() {
    assert_eq!("", apply("after(\"\", _)", ""));
    assert_eq!("", apply("after(foo, _)", ""));
    assert_eq!("", apply("after(foo, _)", "foo"));
    assert_eq!("bar", apply("after(foo, _)", "foobar"));
    assert_eq!("", apply("after(baz, _)", "foobar"));
    assert_apply_error("after(foo, _)", "42");
    assert_apply_error("after(foo, _)", "3.14");
    assert_eval_error("after(foo, [])");
    assert_eval_error("after(foo, {})");
}

#[test]
fn after_regex() {
    assert_eq!("", apply("after(//, _)", ""));
    assert_eq!("", apply("after(/foo/, _)", ""));
    assert_eq!("", apply("after(/foo/, _)", "foo"));
    assert_eq!("bar", apply("after(/foo/, _)", "foobar"));
    assert_eq!("", apply("after(/baz/, _)", "foobar"));
    assert_apply_error("after(/foo/, _)", "42");
    assert_apply_error("after(/foo/, _)", "3.14");
    assert_eval_error("after(/foo/, [])");
    assert_eval_error("after(/foo/, {})");
}
