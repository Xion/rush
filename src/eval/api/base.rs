//! Base API functions.

use eval::{self, Error, Value};
use eval::value::IntegerRepr;


/// Compute the length of given value (an array or a string).
pub fn len(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Array) -> Integer { value.len() as IntegerRepr });
    Err(Error::new(&format!(
        "len() requires string or array, got {}", value.typename()
    )))
}
