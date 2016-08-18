//! Tests for the conversion functions.

use util::*;


#[test]
fn str_() {
    assert_noop_apply("str(_)", "foobar");
    assert_noop_apply("str(_)", "42");
    assert_noop_apply("str(_)", "13.42");
    assert_noop_apply("str(_)", "false");
    assert_eval_error(&format!("str({})", "[]"));
    assert_eval_error(&format!("str({})", "{}"));
}

#[test]
fn int() {
    assert_apply_error("int(_)", "foobar");
    assert_noop_apply("int(_)", "42");
    assert_eq!("13", apply("int(_)", 13.42));
    assert_eq!("0", apply("int(_)", false));
    assert_eval_error(&format!("int({})", "[]"));
    assert_eval_error(&format!("int({})", "{}"));
}

#[test]
fn float() {
    assert_apply_error("float(_)", "foobar");
    assert_eq!("42.0", apply("float(_)", 42));
    assert_noop_apply("float(_)", "13.42");
    assert_eq!("0.0", apply("float(_)", false));
    assert_eval_error(&format!("float({})", "[]"));
    assert_eval_error(&format!("float({})", "{}"));
}

#[test]
fn bool() {
    assert_apply_error("bool(_)", "foobar");
    assert_eq!("true", apply("bool(_)", 42));
    assert_eq!("false", apply("bool(_)", 0));
    assert_eq!("true", apply("bool(_)", 13.42));
    assert_eq!("false", eval(&format!("bool({})", "[]")));
    assert_eq!("false", eval(&format!("bool({})", "{}")));
    assert_eq!("true", eval(&format!("bool({})", "[3]")));
    assert_eq!("true", eval(&format!("bool({})", "{foo: 4}")));
}

#[test]
fn array() {
    assert_eq!(unlines!("f", "o", "o", "b", "a", "r"), apply("array(_)", "foobar"));
    assert_apply_error("array(_)", "42");
    assert_apply_error("array(_)", "13.42");
    assert_apply_error("array(_)", "false");
    assert_eq!(unlines!("3", "4"), eval(&format!("array({})", "[3,4]")));

    // the order of object keys is unspecified
    let keys: Vec<_> = eval(&format!("array({})", "{foo: 3, bar: 4}")).split("\n")
        .map(String::from).collect();
    for &s in &["foo", "bar"] {
        assert!(keys.contains(&String::from(s)));
    }
}

// TODO(xion): tests for csv() function
// TODO(xion): tests for json() function
