//! Root module for actual application logic.

// NOTE: `nom` has to be declared before `log` because both define an error!
// macro, and we want to use the one from `log`.
#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;

extern crate conv;
extern crate fnv;
#[macro_use]
extern crate mopa;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
extern crate unicode_segmentation;


mod eval;
mod parse;

pub use self::eval::Value;
pub use self::eval::value::{BooleanRepr, FloatRepr, IntegerRepr};
pub use self::parse::parse;


use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

use conv::TryFrom;
use rustc_serialize::json::Json;

use self::eval::{Eval, Context, Invoke};


/// Apply the expression to given input stream, line by line,
/// writing to the given output stream.
pub fn map_lines<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    let mut count = 0;
    for line in reader.lines() {
        let line = try!(line);

        // add the input the context, incl. all the various conversions thereof
        context.set("_", line.parse::<Value>().unwrap_or_else(|_| Value::String(line.to_owned())));
        context.set("_b", line.parse::<BooleanRepr>().map(Value::Boolean).unwrap_or(Value::Empty));
        context.set("_f", line.parse::<FloatRepr>().map(Value::Float).unwrap_or(Value::Empty));
        // TODO(xion): consider also trying to parse the line as f64
        // and exposing the rounded version as _i
        context.set("_i", line.parse::<IntegerRepr>().map(Value::Integer).unwrap_or(Value::Empty));
        context.set("_s", Value::String(line.to_owned()));

        let value = context.get("_").unwrap();
        let result = try!(evaluate(&ast, value, &context));
        try!(write_result(&mut writer, result));

        count += 1;
    }

    info!("Processed {} line(s) of input", count);
    Ok(())
}


/// Apply the expression to given input taken as array of lines,
/// writing result to the given output stream.
pub fn apply_lines<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    // parse input lines into a vector of Value objects
    let lines: Vec<_> = BufReader::new(input).lines()
        .map(|r| {
            r.ok().expect("failed to read input line")
                .parse::<Value>().unwrap_or(Value::Empty)
        })
        .filter(|v| *v != Value::Empty)
        .collect();
    let count = lines.len();

    let mut context = Context::new();
    context.set("_", Value::Array(lines));
    let value = context.get("_").unwrap();

    let mut writer = BufWriter::new(output);
    let result = try!(evaluate(&ast, value, &context));
    try!(write_result(&mut writer, result));

    info!("Processed {} line(s) of input", count);
    Ok(())
}

/// Apply the expression to given input taken as single JSON object.
pub fn apply_json<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    // read the input as JSON string and interpret it as Value
    let mut json_string = String::new();
    try!(BufReader::new(input).read_to_string(&mut json_string));
    let json_obj = try!(Json::from_str(&json_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)));

    let mut context = Context::new();
    context.set("_", Value::from(json_obj));
    let value = context.get("_").unwrap();

    let mut writer = BufWriter::new(output);
    let result = try!(evaluate(&ast, value, &context));
    try!(write_result(&mut writer, result));

    info!("Processed {} bytes of JSON", json_string.bytes().len());
    Ok(())
}


// Utility functions.

fn parse_expr(expr: &str) -> io::Result<Box<Eval>> {
    debug!("Using expression: {}", expr);
    parse(expr).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn evaluate<'a>(ast: &Box<Eval>, input: &'a Value, context: &'a Context) -> io::Result<Value> {
    ast.eval(&context)
        .and_then(|result| maybe_apply_result(result, input, &context))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn maybe_apply_result<'a>(result: Value, input: &'a Value, context: &'a Context) -> eval::Result {
    // result might be a function, in which case we will try to apply to original input
    if let Value::Function(func) = result {
        if func.arity() != 1 {
            return Err(eval::Error::new(&format!(
                "output must be an immediate value or a 1-argument function \
                (got {}-argument one)", func.arity())));
        }
        debug!("Result found to be a function, applying it to input");
        return func.invoke(vec![input.clone()], &context);
    }
    Ok(result)
}

fn write_result<W: Write>(output: &mut W, result: Value) -> io::Result<()> {
    let result = try!(String::try_from(result));
    write!(output, "{}\n", result)
}
