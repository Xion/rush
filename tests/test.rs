//! Test crate.

extern crate ap;

use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::string::ToString;


#[test]
fn constant_number() {
    const EXPR: &'static str = "42";
    assert_eq!(EXPR, apply(EXPR, "unused"));
}

#[test]
fn constant_string() {
    const EXPR: &'static str = "foo";
    assert_eq!(EXPR, apply(EXPR, "unused"));
}

#[test]
fn constant_quoted_string() {
    const STRING: &'static str = "foo";
    let expr = &format!("\"{}\"", STRING);
    assert_eq!(STRING, apply(expr, "unused"));
}

#[test]
fn identity() {
    const INPUT: &'static str = "42";
    assert_eq!(INPUT, apply("_", INPUT));
}


// Utility functions.

/// Applies an expression to input given as a string.
///
/// Single- and multiline strings are handled automatically:
/// if the input didn't end with a newline, output won't either.
fn apply(expr: &str, input: &str) -> String {
    let mut extra_newline = false;
    let mut input = input.to_string();
    if !input.ends_with("\n") {
        input.push('\n');
        extra_newline = true;
    }

    let mut output = StringIO::new("");
    ap::apply(expr, StringIO::new(&input), &mut output).unwrap();

    let mut result = output.to_string();
    if extra_newline {
        result.pop();  // remove trailing \n
    }
    result
}

// TODO(xion): this is essentially a minimal implementation of StringIO from Python,
// and it seems useful enough to warrant extracting it as separate crate
// (possibly after generalzing it to MemoryIO that works with arbitrary [u8])
struct StringIO {
    string: String,
    pos: usize,  // in chatacters
}
impl StringIO {
    pub fn new(s: &str) -> Self {
        StringIO{string: s.to_string(), pos: 0}
    }
}
impl Read for StringIO {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.pos < self.string.len() {
            // TODO(xion): write the string in chunks, not all at once
            let bytes = self.string.as_bytes();
            try!(buf.write_all(bytes).map(|_| ()));
            self.pos = self.string.len();
            Ok(bytes.len())
        } else {
            Ok(0)
        }
    }
}
impl Write for StringIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO(xion): ignore the last UTF8 codepoint if it's incomplete
        let s = try!(from_utf8(buf).or(Err(
            io::Error::new(io::ErrorKind::InvalidInput, "malformed UTF8")
        )));
        self.string.push_str(s);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl ToString for StringIO {
    fn to_string(&self) -> String {
        self.string.clone()
    }
}
