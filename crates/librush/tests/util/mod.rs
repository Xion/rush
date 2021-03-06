//! Utility functions used by tests.

mod asserts;
pub mod literals;   // This has to be `pub` because of Rust bug (!)
                    // making ToLiteral trait inaccessible otherwise.
                    // Details: https://github.com/rust-lang/rust/issues/18241

pub use self::asserts::*;
pub use self::literals::*;


use std::collections::HashMap;
use std::io;
use std::str::from_utf8;

use conv::TryFrom;

use rustc_serialize::json::Json;
use rush::{self, Context};


/// Construct a hashmap where key & value is turned into its Owned version
/// prior to inserting to the map.
macro_rules! hashmap_owned {
    {$($key:expr => $value:expr),*} => {
        hashmap!{$($key.to_owned() => $value.to_owned()),*};
    };
}

/// Construct a string literal from given separate string literal lines
/// by joining them all together.
macro_rules! unlines (
    ($($line:expr),*) => ({
        $crate::util::join(&[$($line),*], "\n")
    });
);


/// Join a slice of stringifiable values.
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


// Wrappers around tested code.

/// Evaluate the expression without any input.
pub fn eval(expr: &str) -> String {
    match eval_ex(expr) {
        Ok(output) => output,
        Err(err) => { panic!("eval() error: {}", err); }
    }
}

pub fn eval_ex(expr: &str) -> io::Result<String> {
    let result = try!(rush::eval(expr, &mut Context::new()));
    String::try_from(result)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}


/// Applies an expression to input given as (single line) string.
/// This is a special variant of map_lines().
/// Internally, this calls rush::map_lines.
pub fn apply<T: ToString>(expr: &str, input: T) -> String {
    match apply_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("apply() error: {}", err); }
    }
}

pub fn apply_ex<T: ToString>(expr: &str, input: T) -> io::Result<String> {
    let input = input.to_string();
    assert!(!input.contains("\n"));
    map_lines_ex(expr, input)
}


/// Applies an expression to input given as a string.
///
/// Single- and multiline strings are handled automatically:
/// multiline strings are split into individual lines & mapped over with `expr`.
/// Howeever, if the input didn't end with a newline, output won't either.
///
/// Internally, this calls rush::map_lines.
#[allow(dead_code)]
pub fn map_lines<T: ToString>(expr: &str, input: T) -> String {
    match map_lines_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("map_lines() error: {}", err); }
    }
}

pub fn map_lines_ex<T: ToString>(expr: &str, input: T) -> io::Result<String> {
    let mut extra_newline = false;
    let mut input = input.to_string();
    if !input.ends_with("\n") {
        input.push('\n');
        extra_newline = true;
    }

    let mut output: Vec<u8> = Vec::new();
    try!(rush::map_lines(expr, input.as_bytes(), &mut output));

    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_owned();
    if extra_newline {
        result.pop();  // remove trailing \n
    }
    Ok(result)
}


/// Applies an expression to input given as slice of strings.
/// This input is interpreted as an array by the given expression.
///
/// Internally, this calls rush::apply_lines.
pub fn apply_lines<T: ToString>(expr: &str, input: &[T]) -> String {
    match apply_lines_ex(expr, input) {
        Ok(output) => output,
        Err(err) => { panic!("apply_lines() error: {}", err); }
    }
}

pub fn apply_lines_ex<T: ToString>(expr: &str, input: &[T]) -> io::Result<String> {
    let input  = join(input, "\n");

    let mut output: Vec<u8> = Vec::new();
    try!(rush::apply_lines(expr, input.as_bytes(), &mut output));

    // if the result turns out to be just a single line,
    // remove the trailing \n
    let mut result = try!(
        from_utf8(&output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    ).to_owned();
    if result.ends_with("\n") && result.chars().filter(|c| *c == '\n').count() == 1 {
        result.pop();
    }
    Ok(result)
}
