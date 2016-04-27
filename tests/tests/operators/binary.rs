//! Tests for binary operators.

use util::*;


// TODO(xion): tests for logical operators
// TODO(xion): tests for assignment operators

#[test]
fn compare_less_constants() {
    assert_eval_true("1 < 2");
    assert_eval_true("-5 < 0");
    assert_eval_true("1.5 < 2");
    assert_eval_true("8 < 10.0");
    assert_eval_true("-3.14 < 3.14");
    assert_eval_false("1 < 1");
    assert_eval_false("0 < -10");
    assert_eval_error("0 < foo");
    assert_eval_error("foo < 42");
    assert_eval_error("bar < true");
    assert_eval_error("[] < []");
    assert_eval_error("{} < {}");
}
// TODO(xion): compare_less_inputs

#[test]
fn compare_less_or_equal_constants() {
    assert_eval_true("1 <= 2");
    assert_eval_true("-5 <= 0");
    assert_eval_true("1.5 <= 2");
    assert_eval_true("8 <= 10.0");
    assert_eval_true("-3.14 <= 3.14");
    assert_eval_true("1 <= 1");
    assert_eval_false("0 <= -10");
    assert_eval_false("-8 <= -12");
    assert_eval_error("0 <= foo");
    assert_eval_error("foo <= 42");
    assert_eval_error("bar <= true");
    assert_eval_error("[] <= []");
    assert_eval_error("{} <= {}");
}
// TODO(xion): compare_less_or_equal_inputs

#[test]
fn compare_greater_constants() {
    assert_eval_true("2 > 1");
    assert_eval_true("0 > -5");
    assert_eval_true("2 > 1.5");
    assert_eval_true("10.0 > 8");
    assert_eval_true("3.14 > -3.14");
    assert_eval_false("1 > 1");
    assert_eval_false("-10 > 0");
    assert_eval_false("-12 > -8");
    assert_eval_error("0 > foo");
    assert_eval_error("foo > 42");
    assert_eval_error("bar > true");
    assert_eval_error("[] > []");
    assert_eval_error("{} > {}");
}
// TODO(xion): compare_greater_inputs

#[test]
fn compare_greater_or_equal_constants() {
    assert_eval_true("2 >= 1");
    assert_eval_true("0 >= -5");
    assert_eval_true("2 >= 1.5");
    assert_eval_true("10.0 >= 8");
    assert_eval_true("3.14 >= -3.14");
    assert_eval_true("1 >= 1");
    assert_eval_false("-10 >= 0");
    assert_eval_false("-12 >= -8");
    assert_eval_error("0 >= foo");
    assert_eval_error("foo >= 42");
    assert_eval_error("bar >= true");
    assert_eval_error("[] >= []");
    assert_eval_error("{} >= {}");
}
// TODO(xion): compare_greater_or_equal_inputs

#[test]
fn compare_equal_constants() {
    assert_eval_false("2 == 1");
    assert_eval_false("0 == -5");
    assert_eval_false("2 == 1.5");
    assert_eval_false("10.0 == 8");
    assert_eval_false("3.14 == -3.14");
    assert_eval_false("-10 == 0");
    assert_eval_false("-12 == -8");
    assert_eval_true("1 == 1");
    assert_eval_true("2.0 == 2");
    assert_eval_true("3.0 == 3.0");
    assert_eval_true("4 == 4.0");
    assert_eval_true("[] == []");
    assert_eval_true("{} == {}");
    assert_eval_error("0 == foo");
    assert_eval_error("foo == 42");
    assert_eval_error("bar == true");
}
// TODO(xion): compare_equal_inputs

#[test]
fn compare_not_equal_constants() {
    assert_eval_true("2 != 1");
    assert_eval_true("0 != -5");
    assert_eval_true("2 != 1.5");
    assert_eval_true("10.0 != 8");
    assert_eval_true("3.14 != -3.14");
    assert_eval_true("-10 != 0");
    assert_eval_true("-12 != -8");
    assert_eval_false("1 != 1");
    assert_eval_false("2.0 != 2");
    assert_eval_false("3.0 != 3.0");
    assert_eval_false("4 != 4.0");
    assert_eval_false("[] != []");
    assert_eval_false("{} != {}");
    assert_eval_error("0 != foo");
    assert_eval_error("foo != 42");
    assert_eval_error("bar != true");
}
// TODO(xion): compare_not_equal_inputs

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
