//! Tests for the conversion functions.

use util::*;


#[test]
fn str_() {
    assert_noop_apply("str(_)", "foobar");
    assert_noop_apply("str(_)", &42.to_string());
    assert_noop_apply("str(_)", &13.42.to_string());
    assert_noop_apply("str(_)", &false.to_string());
    assert_eval_error(&format!("str({})", "[]"));
    assert_eval_error(&format!("str({})", "{}"));
}

#[test]
fn int() {
    assert_apply_error("int(_)", "foobar");
    assert_noop_apply("int(_)", &42.to_string());
    assert_eq!("13", apply("int(_)", &13.42.to_string()));
    assert_eq!("0", apply("int(_)", &false.to_string()));
    assert_eval_error(&format!("int({})", "[]"));
    assert_eval_error(&format!("int({})", "{}"));
}

#[test]
fn float() {
    assert_apply_error("float(_)", "foobar");
    assert_eq!("42.0", apply("float(_)", &42.to_string()));
    assert_noop_apply("float(_)", &13.42.to_string());
    assert_eq!("0.0", apply("float(_)", &false.to_string()));
    assert_eval_error(&format!("float({})", "[]"));
    assert_eval_error(&format!("float({})", "{}"));
}

#[test]
fn bool() {
    assert_apply_error("bool(_)", "foobar");
    assert_eq!("true", apply("bool(_)", &42.to_string()));
    assert_eq!("false", apply("bool(_)", &0.to_string()));
    assert_eq!("true", apply("bool(_)", &13.42.to_string()));
    assert_eq!("false", eval(&format!("bool({})", "[]")));
    assert_eq!("false", eval(&format!("bool({})", "{}")));
    assert_eq!("true", eval(&format!("bool({})", "[3]")));
    assert_eq!("true", eval(&format!("bool({})", "{foo: 4}")));
}
