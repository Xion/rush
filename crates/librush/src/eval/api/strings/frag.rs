//! API for blowing strings into fragments and putting them back together.

use regex::Regex;

use eval::{self, Error, Value};
use eval::api::conv::str_;
use eval::value::{ArrayRepr, StringRepr};


lazy_static!{
    static ref WORD_SEP: Regex = Regex::new(r"\s+").unwrap();
    static ref LINE_SEP: Regex = Regex::new("\r?\n").unwrap();
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
            // TODO: include the error message of the offending element's conversion
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


/// Split a string by given string delimiter.
/// Returns an array of strings.
// TODO(xion): introduce optional third parameter, maxsplit
pub fn split(delim: Value, string: Value) -> eval::Result {
    eval2!((delim: &String, string: &String) -> Array {
        string.split(delim).map(StringRepr::from).map(Value::String).collect()
    });
    eval2!((delim: &Regex, string: &String) -> Array {
        do_regex_split(delim, string)
    });
    mismatch!("split"; ("string", "string") | ("regex", "string") => (delim, string))
}

/// Split a string into array of words.
pub fn words(string: Value) -> eval::Result {
    eval1!((string: &String) -> Array { do_regex_split(&WORD_SEP, string) });
    mismatch!("words"; ("string") => (string))
}

/// Split a string into array of lines.
pub fn lines(string: Value) -> eval::Result {
    eval1!((string: &String) -> Array { do_regex_split(&LINE_SEP, string) });
    mismatch!("lines"; ("string") => (string))
}


// Utility functions

#[inline]
fn do_regex_split(delim: &Regex, string: &str) -> ArrayRepr {
    delim.split(string).map(StringRepr::from).map(Value::String).collect()
}
