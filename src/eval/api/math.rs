//! Math functions.

use std::fmt::Display;

use rand::random;

use eval::{self, Error, Value};
use eval::value::FloatRepr;


/// Compute the absolute value of a number.
pub fn abs(value: Value) -> eval::Result {
    eval1!(value : Integer { value.abs() });
    eval1!(value : Float { value.abs() });
    Err(Error::new(&format!(
        "abs() requires a number, got {}", value.typename()
    )))
}

/// Compute the signum function.
pub fn sgn(value : Value) -> eval::Result {
    eval1!(value : Integer { value.signum() });
    eval1!(value : Float { value.signum() });
    Err(Error::new(&format!(
        "sgn() requires a number, got {}", value.typename()
    )))
}

/// Compute a square root of a number.
pub fn sqrt(value : Value) -> eval::Result {
    fn ensure_nonnegative<T>(x : T) -> Result<T, Error>
        where T: Default + Display + PartialOrd
    {
        // TODO(xion): use the Zero trait instead of Default
        // when it's available in stable Rust
        if x >= T::default() {
            Ok(x)
        } else {
            Err(Error::new(&format!(
                "sqrt() requires a non-negative number, got {}", x
            )))
        }
    }

    eval1!((value: Integer) -> Float {
        (try!(ensure_nonnegative(value)) as FloatRepr).sqrt()
    });
    eval1!(value : Float {
        try!(ensure_nonnegative(value)).sqrt()
    });

    Err(Error::new(&format!(
        "sqrt() requires a number, got {}", value.typename()
    )))
}

/// Generate a random floating point number from the 0..1 range.
pub fn rand() -> eval::Result {
    Ok(Value::Float(random()))
}
