//! API that's available out-of-the-box to the expressions.
//! It is essentially the standard library of the language.

use rand as _rand;

use eval;
use super::model::Value;


/// Generate a random floating point number from the 0..1 range.
pub fn rand() -> eval::Result {
    Ok(Value::Float(_rand::random::<f64>()))
}
