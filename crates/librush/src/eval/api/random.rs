//! Functions related to random number generation.

use rand::{self, Rng};

use eval::{self, Value};


/// Generate a random floating point number from the 0..1 range.
#[inline]
pub fn rand_() -> eval::Result {
    Ok(Value::Float(rand::random()))
}


/// Shuffle the elements of an array or characters in a string.
pub fn shuffle(value: Value) -> eval::Result {
    if value.is_array() {
        let mut array = value.unwrap_array();
        rand::thread_rng().shuffle(&mut array);
        return Ok(Value::Array(array));
    }

    if value.is_string() {
        let mut chars: Vec<_> = value.unwrap_string().chars().collect();
        rand::thread_rng().shuffle(&mut chars);

        let mut result = String::with_capacity(chars.len());
        for c in chars.into_iter() {
            result.push(c);
        }
        return Ok(Value::String(result));
    }

    mismatch!("shuffle"; ("array") | ("string") => (value))
}


/// Pick a sample of given size among the values from a collection.
/// Returns the array of picked elements.
pub fn sample(size: Value, source: Value) -> eval::Result {
    if size.is_integer() {
        let mut rng = rand::thread_rng();

        if source.is_array() {
            let size = size.unwrap_integer() as usize;
            return Ok(Value::Array(
                rand::sample(&mut rng, source.unwrap_array().into_iter(), size)
            ));
        }
        if source.is_string() {
            let size = size.unwrap_integer() as usize;
            let mut result = String::with_capacity(size);
            for c in rand::sample(&mut rng, source.unwrap_string().chars(), size) {
                result.push(c);
            }
            return Ok(Value::String(result));
        }
        if source.is_object() {
            let size = size.unwrap_integer() as usize;
            return Ok(Value::Array(
                rand::sample(&mut rng,
                    source.unwrap_object().into_iter().map(|(_, v)| v),
                    size)
            ));
        }
    }
    mismatch!("sample";
        ("integer", "array") | ("integer", "string") | ("integer", "object")
        => (size, source))
}
