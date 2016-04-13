//! API for blowing strings into fragments and putting them back together.

use eval::{self, Error, Value};
use eval::api::conv::str_;
use eval::value::StringRepr;


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
