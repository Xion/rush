//! Assertion functions.

use super::{apply, apply_ex, apply_lines_ex, eval, eval_ex};


// TODO(xion): allow for more fine grained error assertions

pub fn assert_noop_eval(expr: &str) {
    assert_eq!(expr, eval(expr));
}

pub fn assert_noop_apply(expr: &str, input: &str) {
    assert_eq!(input, apply(expr, input));
}

pub fn assert_eval_error(expr: &str) {
    assert!(eval_ex(expr).is_err(),
        "Expression `{}` didn't cause an error!", expr);
}

pub fn assert_eval_true(expr: &str) {
    let result = eval(expr);
    let result_bool = result.parse::<bool>().expect(&format!(
        "Couldn't interpret result of `{}` as boolean: {}", expr, result
    ));
    assert!(result_bool, "unexpectedly false: {}", expr);
}

pub fn assert_eval_false(expr: &str) {
    let result = eval(expr);
    let result_bool = result.parse::<bool>().expect(&format!(
        "Couldn't interpret result of `{}` as boolean: {}", expr, result
    ));
    assert!(!result_bool, "unexpectedly true: {}", expr);
}

pub fn assert_apply_error<T: ToString>(expr: &str, input: T) {
    let input = &input.to_string();
    assert!(apply_ex(expr, input).is_err(),
        "Mapping `{}` for input `{}` didn't cause an error!", expr, input);
}

pub fn assert_apply_lines_error<T: ToString>(expr: &str, input: &[T]) {
    assert!(apply_lines_ex(expr, input).is_err(),
        "Reducing `{}` on input `{}` didn't cause an error!");
}
