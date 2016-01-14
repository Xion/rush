//! Root module for actual application logic.

// NOTE: `nom` has to be declared before `log` because both define an error!
// macro, and we want to use the one from `log`.
#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;

extern crate rand;
extern crate regex;

mod ast;
mod eval;
mod parse;

pub use self::eval::Value;
pub use self::parse::parse;


use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

use self::eval::{Eval, Context};


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
        update_context(&mut context, &line);

        let result = try!(ast.eval(&context)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
        try!(write!(writer, "{}\n", result));

        count += 1;
    }

    info!("Processed {} line(s) of input", count);
    Ok(())
}

fn update_context(ctx: &mut Context, input: &str) {
    ctx.set_var("_", input.parse::<Value>().unwrap_or_else(|_| Value::String(input.to_string())));
    ctx.set_var("_b", input.parse::<bool>().map(Value::Boolean).unwrap_or(Value::Empty));
    ctx.set_var("_f", input.parse::<f64>().map(Value::Float).unwrap_or(Value::Empty));
    // TODO(xion): consider also trying to parse the input as f64
    // and exposing the rounded version as _i
    ctx.set_var("_i", input.parse::<i64>().map(Value::Integer).unwrap_or(Value::Empty));
    ctx.set_var("_s", Value::String(input.to_string()));
}
