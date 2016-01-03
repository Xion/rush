//! Root module for actual application logic.

// NOTE: `nom` has to be declared before `log` because both define an error!
// macro, and we want to use the one from `log`.
#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;

extern crate regex;

mod ast;
mod eval;
mod parse;


use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

use self::eval::{Eval, Context, Value};
use self::parse::parse;


/// Apply the expression to given input stream,
/// writing to the given output stream.
pub fn apply<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> Result<(), io::Error> {
    debug!("Using expression: {}", expr);

    let ast = try!(parse(expr)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)));

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    let mut count = 0;
    for line in reader.lines() {
        let line = try!(line);
        context.set_var("_",
            line.parse::<Value>().unwrap_or_else(|_| Value::String(line)));

        let result = try!(ast.eval(&context)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
        try!(write!(writer, "{}\n", result));

        count += 1;
    }

    info!("Processed {} line(s) of input", count);
    Ok(())
}
