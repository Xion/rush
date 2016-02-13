//! Utility functions used by tests.

// TODO(xion): this is a utility module, but it's executed as if it was
// another module with tests (and therefore requires repetition of
// these `extern crate` declarations); fix this
extern crate ap;
extern crate rustc_serialize;


use std::collections::HashMap;
use std::io;
use std::str::from_utf8;

use rustc_serialize::json::Json;


pub fn join<T: ToString>(array: &[T], sep: &str) -> String {
    array.iter().map(T::to_string).collect::<Vec<_>>().join(sep)
}

/// Parse JSON containing only string values.
pub fn parse_json_stringmap(json: &str) -> HashMap<String, String> {
    let json = Json::from_str(json).expect("failed to parse as JSON");
    match json {
        Json::Object(o) => o.into_iter()
            .map(|(k, v)| (k, v.as_string().unwrap().to_owned())).collect(),
        _ => { panic!("expected a JSON object literal") },
    }
}


// Assertions.
// TODO(xion): allow for more fine grained error assertions

pub fn assert_noop_eval(expr: &str) {
    assert_eq!(expr, eval(expr));
}

pub fn assert_noop_apply(expr: &str, input: &str) {
    assert_eq!(input, apply(expr, input));
}

pub fn assert_eval_error(expr: &str) {
    if !eval_ex(expr).is_err() {
        panic!("Expression `{}` didn't cause an error!", expr);
    }
}

pub fn assert_eval_true(expr: &str) {
    let result = eval(expr);
    assert!(result.parse::<bool>().expect(&format!(
        "Couldn't interpret result of `{}` as boolean: {}", expr, result
    )));
}

pub fn assert_eval_false(expr: &str) {
    let result = eval(expr);
    assert!(!result.parse::<bool>().expect(&format!(
        "Couldn't interpret result of `{}` as boolean: {}", expr, result
    )));
}

pub fn assert_apply_error(expr: &str, input: &str) {
    if !apply_ex(expr, input).is_err() {
        panic!("Mapping `{}` for input `{}` didn't cause an error!", expr, input);
    }
}

pub fn assert_apply_lines_error<'a>(expr: &str, input: &'a [&'a str]) {
    if !apply_lines_ex(expr, input).is_err() {
        panic!("Reducing `{}` on input `{}` didn't cause an error!");
    }
}


// Wrappers around tested code.

/// Evaluate the expression without any input.
pub fn eval(expr: &str) -> String {
    match eval_ex(expr) {
        Ok(output) => output,
        Err(err) => { panic!("eval() error: {}", err); }
    }
}

pub fn eval_ex(expr: &str) -> io::Result<String> {
    apply_ex(expr, "unused")
}


/// Applies an expression to input given as (single line) string.
/// This is a special variant of map_lines().
/// Internally, this calls ap::map_lines.
pub fn apply(expr: &str, input: &str) -> String {
    match apply_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("apply() error: {}", err); }
    }
}

pub fn apply_ex(expr: &str, input: &str) -> io::Result<String> {
    assert!(!input.contains("\n"));
    map_lines_ex(expr, input)
}


/// Applies an expression to input given as a string.
///
/// Single- and multiline strings are handled automatically:
/// multiline strings are split into individual lines & mapped over with `expr`.
/// Howeever, if the input didn't end with a newline, output won't either.
///
/// Internally, this calls ap::map_lines.
#[allow(dead_code)]
pub fn map_lines(expr: &str, input: &str) -> String {
    match map_lines_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("map_lines() error: {}", err); }
    }
}

pub fn map_lines_ex(expr: &str, input: &str) -> io::Result<String> {
    let mut extra_newline = false;
    let mut input = input.to_string();
    if !input.ends_with("\n") {
        input.push('\n');
        extra_newline = true;
    }

    let mut output: Vec<u8> = Vec::new();
    try!(ap::map_lines(expr, input.as_bytes(), &mut output));

    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_string();
    if extra_newline {
        result.pop();  // remove trailing \n
    }
    Ok(result)
}


/// Applies an expression to input given as slice of strings.
/// This input is interpreted as an array by the given expression.
///
/// Internally, this calls ap::apply_lines.
pub fn apply_lines<'a>(expr: &str, input: &'a [&'a str]) -> String {
    match apply_lines_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("apply_lines() error: {}", err); }
    }
}

pub fn apply_lines_ex<'a>(expr: &str, input: &'a [&'a str]) -> io::Result<String> {
    let input = input.join("\n").to_string();

    let mut output: Vec<u8> = Vec::new();
    try!(ap::apply_lines(expr, input.as_bytes(), &mut output));

    // if the result turns out to be just a single line,
    // remove the trailing \n
    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_string();
    if result.chars().filter(|c| *c == '\n').count() == 1 {
        result.pop();
    }
    Ok(result)
}


/// Return the string representation of Value::Empty.
pub fn empty() -> String {
    format!("{}", ap::Value::Empty)
}
