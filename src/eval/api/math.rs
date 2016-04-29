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
#[inline(always)]
pub fn rand() -> eval::Result {
    Ok(Value::Float(random()))
}

/// The exponential function.
pub fn exp(value : Value) -> eval::Result {
    eval1!((value : Integer) -> Float { (value as FloatRepr).exp() });
    eval1!(value : Float { value.exp() });
    Err(Error::new(&format!(
        "exp() requires a number, got {}", value.typename()
    )))
}

/// Natural logarithm (with respect to base 'e').
pub fn ln(value : Value) -> eval::Result {
    eval1!((value : Integer) -> Float { (value as FloatRepr).ln() });
    eval1!(value : Float { value.ln() });
    Err(Error::new(&format!(
        "ln() requires a number, got {}", value.typename()
    )))
}


// Rounding

/// Round a number down.
pub fn floor(value: Value) -> eval::Result {
    eval1!(value : Integer { value });
    eval1!(value : Float { value.floor() });
    Err(Error::new(&format!(
        "floor() requires a number, got {}", value.typename()
    )))
}

/// Round a number up.
pub fn ceil(value: Value) -> eval::Result {
    eval1!(value : Integer { value });
    eval1!(value : Float { value.ceil() });
    Err(Error::new(&format!(
        "ceil() requires a number, got {}", value.typename()
    )))
}

/// Round a number to the nearest integer.
pub fn round(value : Value) -> eval::Result {
    eval1!(value : Integer { value });
    eval1!(value : Float { value.round() });
    Err(Error::new(&format!(
        "round() requires a number, got {}", value.typename()
    )))
}

/// Return the integer part of the number.
pub fn trunc(value : Value) -> eval::Result {
    eval1!(value : Integer { value });
    eval1!(value : Float { value.trunc() });
    Err(Error::new(&format!(
        "trunc() requires a number, got {}", value.typename()
    )))
}


// Numeric bases

/// Convert an integer to a binary string.
pub fn bin(value: Value) -> eval::Result {
    eval1!((value : Integer) -> String { format!("{:b}", value) });
    Err(Error::new(&format!(
        "bin() requires a number, got {}", value.typename()
    )))
}

/// Convert an integer to an octal string.
pub fn oct(value: Value) -> eval::Result {
    eval1!((value : Integer) -> String { format!("{:o}", value) });
    Err(Error::new(&format!(
        "oct() requires a number, got {}", value.typename()
    )))
}

/// Convert an integer to a hexidecimal string.
pub fn hex(value: Value) -> eval::Result {
    eval1!((value : Integer) -> String { format!("{:x}", value) });
    Err(Error::new(&format!(
        "hex() requires a number, got {}", value.typename()
    )))
}
