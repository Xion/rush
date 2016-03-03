//! Tests for "trialers", i.e. final parts of atoms: subscripts & function calls.

use util::*;


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
    assert_eq!("foo", apply_lines("_[0]", INPUT));
    assert_eq!("bar", apply_lines("_[1]", INPUT));
    assert_eq!("foo", apply_lines("[_][0][0]", INPUT));
    assert_eq!("other", apply_lines("[_, [other]][1][0]", INPUT));
    assert_apply_lines_error("_[42]", INPUT);
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
    assert_eq!("he\n\no", eval("split(l, hello)"));
}

#[test]
fn function_call_2args_input() {
    assert_eq!("he\n\no", apply("split(l, _)", "hello"));
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
