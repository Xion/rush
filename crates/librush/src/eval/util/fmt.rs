//! String formatting at runtime.
//! Poached from https://github.com/panicbit/monster & refined a little.

#![allow(dead_code)]


use std::error;
use std::fmt::{self, Display, Write};
use std::result;


/// Format a string.
/// The format syntax is similar to the one used by `std::fmt`,
/// but very limited at the moment.
///
/// # Example
///
/// ```
/// let fmt = "You see {{{}}} tiny {}";
/// let result = format(fmt , &[&10, &"monsters"]);
///
/// assert_eq!(result.unwrap(), "You see {10} tiny monsters");
/// ```
pub fn format(fmt: &str, args: &[&Display]) -> Result<String> {
    let mut buffer = String::with_capacity(fmt.len());
    try!(write_format(&mut buffer, fmt, args));
    Ok(buffer)
}

/// Same as `format` but writes to a generic buffer instead.
pub fn write_format<W: Write>(buffer: &mut W, fmt: &str, args: &[&Display]) -> Result<()> {
    let mut args = args.iter();
    let mut state = Normal;

    for ch in fmt.chars() {
        match state {
            Normal => match ch {
                '{' => state = LeftBrace,
                '}' => state = RightBrace,
                _   => try!(buffer.write_char(ch))
            },
            LeftBrace => match ch {
                // An escaped '{'
                '{' => {
                    try!(buffer.write_char(ch));
                    state = Normal
                },
                // An escaped '}'
                '}' => {
                    match args.next() {
                        Some(arg) => try!(write!(buffer, "{}", arg)),
                        None => return Err(Error::NotEnoughArgs)
                    };
                    state = Normal
                },
                 // No named placeholders allowed
                _  => return Err(Error::UnexpectedChar)
            },
            RightBrace => match ch {
                '}' => {
                    try!(buffer.write_char(ch));
                    state = Normal
                },
                // No standalone right brace allowed
                _ => return Err(Error::UnexpectedRightBrace)
            }
        }
    }
    Ok(())
}


enum State {
    Normal,
    LeftBrace,
    RightBrace,
}
use self::State::*;


// Error & result type

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
pub enum Error {
    NotEnoughArgs,
    UnexpectedChar,
    UnexpectedRightBrace,
    Unkown
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "Formatting error: {}", self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotEnoughArgs => "not enough arguments passed",
            Error::UnexpectedChar => "unexpected character",
            Error::UnexpectedRightBrace => "unexpected right brace",
            Error::Unkown => "unknown error"
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error::Unkown
    }
}
