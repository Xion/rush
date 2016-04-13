//! String API available to expressions.

mod subst;

pub use self::subst::*;


use std::char;
use std::error::Error as StdError;  // just for its description() method
use std::fmt::Display;
use std::str::from_utf8;

use unicode_segmentation::UnicodeSegmentation;

use eval::{self, Error, Value};
use eval::value::{IntegerRepr, StringRepr};
use eval::util::fmt::format;
use super::conv::str_;


/// Returns a one-character string with the character of given ordinal value.
pub fn chr(value: Value) -> eval::Result {
    eval1!((value: Integer) -> String where (value >= 0) {{
        let ord = value as u32;
        let ch = try!(char::from_u32(ord)
            .ok_or_else(|| Error::new(&format!(
                "invalid character ordinal: {}", ord
            ))));
        let mut result = String::with_capacity(1);
        result.push(ch);
        result
    }});
    Err(Error::new(&format!(
        "chr() expects a positive integer, got {}", value.typename()
    )))
}

/// Returns the ordinal value for a single character in a string.
pub fn ord(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer {
        match value.len() {
            1 => value.chars().next().unwrap() as IntegerRepr,
            len@_ => return Err(Error::new(&format!(
                "ord() requires string of length 1, got {}", len
            ))),
        }
    });
    Err(Error::new(&format!(
        "ord() expects a string, got {}", value.typename()
    )))
}


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
    eval2!((delim: &Regex, string: &String) -> Array {
        delim.split(&string).map(StringRepr::from).map(Value::String).collect()
    });

    Err(Error::new(&format!(
        "split() expects string/regex delimiter and string to split, got: {}, {}",
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

/// Peforms string formatting a'la Python str.format().
pub fn format_(fmt: Value, arg: Value) -> eval:: Result {
    if let Value::String(fmt) = fmt {
        let mut args: Vec<&Display> = Vec::new();

        match &arg {
            &Value::Boolean(..) |
            &Value::Integer(..) |
            &Value::Float(..) |
            &Value::String(..) => args.push(&arg),
            &Value::Array(ref a) => {
                args = a.iter().map(|v| v as &Display).collect();
            },
            &Value::Object(..) => {
                // TODO(xion): Object should be possible but the formatting code
                // doesn't support named placeholders yet :(
                return Err(Error::new(
                    "objects are not supported as string formatting arguments"
                ));
            },
            _ => return Err(Error::new(&format!(
                "invalid argument for string formatting: {}", arg.typename()
            ))),
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
    eval2!((needle: &Regex, haystack: &String) -> String {
        match needle.find(&haystack) {
            Some((index, _)) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[0..index]).unwrap()
            ),
            _ => String::new(),
        }
    });

    Err(Error::new(&format!(
        "before() expects two strings, or regex and string, got {} and {}",
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
    eval2!((needle: &Regex, haystack: &String) -> String {
        match needle.find(&haystack) {
            Some((_, index)) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[index..]).unwrap()
            ),
            _ => String::new(),
        }
    });

    Err(Error::new(&format!(
        "after() expects two strings, or regex and string, got {} and {}",
        needle.typename(), haystack.typename()
    )))
}

/// Trim the string from whitespace characters at both ends.
pub fn trim(string: Value) -> eval::Result {
    eval1!(string : &String { string.trim().to_owned() });
    Err(Error::new(&format!(
        "trim() requires a string, got {}", string.typename()
    )))
}
