//! String API available to expressions.

use std::error::Error as _Error;  // just for description() method
use std::fmt::Display;
use std::str::from_utf8;

use unicode_segmentation::UnicodeSegmentation;

use eval::{self, Error, Value};
use eval::value::StringRepr;
use eval::util::fmt::format;
use super::conv::str_;


/// Reverse the characters in a string.
pub fn rev(string: Value) -> eval::Result {
    eval1!(string : &String {
        string.graphemes(/* extended grapheme clusters */ true)
            .rev()
            .collect::<Vec<_>>().join("")
    });
    Err(Error::new(&format!(
        "rev() requires a string, got {}", string.typename()
    )))
}

/// Split a string by given string delimiter.
/// Returns an array of strings.
pub fn split(delim: Value, string: Value) -> eval::Result {
    eval2!((delim: &String, string: &String) -> Array {
        string.split(delim).map(StringRepr::from).map(Value::String).collect()
    });

    // TODO(xion): split with regex as delimiter (also as / operator)
    Err(Error::new(&format!(
        "split() expects string delimiter and string to split, got: {}, {}",
        string.typename(), delim.typename()
    )))
}

/// Join an array of values into a single delimited string.
pub fn join(delim: Value, array: Value) -> eval::Result {
    let delim_type = delim.typename();
    let array_type = array.typename();

    if let (Value::String(d), Value::Array(a)) = (delim, array) {
        let elem_count = a.len();
        let strings: Vec<_> =  a.into_iter()
            .map(str_).filter(Result::is_ok)
            .map(Result::unwrap).map(Value::unwrap_string)
            .collect();
        let error_count = strings.len() - elem_count;
        if error_count == 0 {
            return Ok(Value::String(strings.join(&d)));
        } else {
            return Err(Error::new(&format!(
                "join() failed to stringify {} element(s) of the input array",
                error_count)));
        }
    }

    Err(Error::new(&format!(
        "join() expects a string and an array, got: {}, {}",
        delim_type, array_type
    )))
}

/// Substitute a given string ("needle") with another ("replacement")
/// within given text ("haystack").
/// Returns the text after substitution has been made.
pub fn sub(needle: Value, replacement: Value, haystack: Value) -> eval::Result {
    if let (&Value::String(ref n),
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        return Ok(Value::String(h.replace(n, r)));
    }
    Err(Error::new(&format!(
        "sub() expects three strings, got: {}, {}, {}",
        needle.typename(), replacement.typename(), haystack.typename()
    )))
}

/// Peforms string formatting a'la Python str.format().
pub fn format_(fmt: Value, arg: Value) -> eval:: Result {
    if let Value::String(fmt) = fmt {
        let mut args: Vec<&Display> = Vec::new();

        match &arg {
            &Value::Empty |
            &Value::Symbol(..) |
            &Value::Function(..) => return Err(Error::new(&format!(
                "invalid argument for string formatting: {}", arg.typename()
            ))),
            &Value::Object(..) => {
                // TODO(xion): Object should be possible but the formatting code
                // doesn't support named placeholders yet :(
                return Err(Error::new(
                    "objects are not supported as string formatting arguments"
                ));
            },
            &Value::Array(ref a) => {
                args = a.iter().map(|v| v as &Display).collect();
            },
            _ => args.push(&arg),
        }

        return format(&fmt, &args)
            .map(Value::String)
            .map_err(|e| Error::new(&format!(
                "string formatting error: {}", e.description()
            )));
    }

    Err(Error::new(&format!(
        "format() expects a format string, got {}", fmt.typename()
    )))
}

/// Return part of a string ("haystack") before given one ("needle"),
/// or empty string if not found.
pub fn before(needle: Value, haystack: Value) -> eval::Result {
    eval2!((needle: &String, haystack: &String) -> String {
        match haystack.find(&needle as &str) {
            Some(index) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[0..index]).unwrap()
            ),
            _ => String::new(),
        }
    });
    Err(Error::new(&format!(
        "before() expects two strings, got {} and {}",
        needle.typename(), haystack.typename()
    )))
}

/// Return part of a string ("haystack") after given one ("needle"),
/// or empty string if not found.
pub fn after(needle: Value, haystack: Value) -> eval::Result {
    eval2!((needle: &String, haystack: &String) -> String {
        match haystack.find(&needle as &str) {
            Some(index) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[index + needle.len()..]).unwrap()
            ),
            _ => String::new(),
        }
    });
    Err(Error::new(&format!(
        "after() expects two strings, got {} and {}",
        needle.typename(), haystack.typename()
    )))
}
