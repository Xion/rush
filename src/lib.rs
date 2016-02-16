//! Root module for actual application logic.

// NOTE: `nom` has to be declared before `log` because both define an error!
// macro, and we want to use the one from `log`.
#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;

extern crate rand;
extern crate regex;
extern crate rustc_serialize;


mod eval;
mod parse;

pub use self::eval::Value;
pub use self::parse::parse;


use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

use rustc_serialize::json::Json;

use self::eval::{Eval, Context};


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
        update_context(&mut context, &line);

        let result = try!(eval(&ast, &context));
        try!(write!(writer, "{}\n", result));

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

    let mut writer = BufWriter::new(output);
    let result = try!(eval(&ast, &context));
    try!(write!(writer, "{}\n", result));

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
    let value = Value::from(json_obj);

    let mut context = Context::new();
    context.set("_", value);

    let mut writer = BufWriter::new(output);
    let result = try!(eval(&ast, &context));
    try!(write!(writer, "{}\n", result));

    Ok(())
}


// Utility functions.

fn parse_expr(expr: &str) -> io::Result<Box<Eval>> {
    debug!("Using expression: {}", expr);
    parse(expr).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn update_context(ctx: &mut Context, input: &str) {
    ctx.set("_", input.parse::<Value>().unwrap_or_else(|_| Value::String(input.to_owned())));
    ctx.set("_b", input.parse::<bool>().map(Value::Boolean).unwrap_or(Value::Empty));
    ctx.set("_f", input.parse::<f64>().map(Value::Float).unwrap_or(Value::Empty));
    // TODO(xion): consider also trying to parse the input as f64
    // and exposing the rounded version as _i
    ctx.set("_i", input.parse::<i64>().map(Value::Integer).unwrap_or(Value::Empty));
    ctx.set("_s", Value::String(input.to_owned()));
}

fn eval(ast: &Box<Eval>, ctx: &Context) -> io::Result<Value> {
    ast.eval(&ctx).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
