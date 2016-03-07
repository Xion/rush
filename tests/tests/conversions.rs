//! Tests for input conversion constructs.
//!
//! Note that this is only about the _i, _b, etc. constructs.
//! Tests for the type conversion API are in tests::api::conv module.

use util::*;


#[test]
fn identity_on_int() {
    assert_noop_apply("_", "42");
}

#[test]
fn identity_on_string() {
    assert_noop_apply("_", "foo");
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
    assert_apply_error("_i", "42.42");
    assert_apply_error("_i", "true");
    assert_apply_error("_i", "foo");
}

#[test]
fn input_conversion_float() {
    assert_noop_apply("_f", "42.42");
    assert_eq!("42.0", apply("_f", "42"));
    assert_apply_error("_f", "true");
    assert_apply_error("_f", "foo");
}

#[test]
fn input_conversion_boolean() {
    assert_noop_apply("_b", "true");
    assert_noop_apply("_b", "false");
    assert_apply_error("_b", "42");
    assert_apply_error("_b", "42.42");
    assert_apply_error("_b", "foo");
}

#[test]
fn input_conversion_string() {
    assert_noop_apply("_s", "42");
    assert_noop_apply("_s", "42.42");
    assert_noop_apply("_s", "true");
    assert_noop_apply("_s", "foo");
}

// TODO(xion): test str(), int(), etc. functions
