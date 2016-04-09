//! Convenience wrappers around parsing and evaluation.

use std::iter::Iterator;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::u8;

use conv::TryFrom;

use super::eval::{self, Eval, Context, Invoke, Value};
use super::eval::value::{BooleanRepr, FloatRepr, IntegerRepr};
use super::parse::parse;


/// Apply the expresion to a complete input stream, processed as single string,
/// writing to the given output stream.
pub fn apply_string<R: Read, W: Write>(expr: &str, input: R, mut output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    let mut reader = BufReader::new(input);
    let mut input = String::new();
    let byte_count = try!(reader.read_to_string(&mut input));

    let mut context = Context::new();
    update_context(&mut context, &input);

    let value = context.get("_").unwrap();
    let result = try!(evaluate(&ast, value, &context));
    try!(write_result(&mut output, &result));

    info!("Processed {} character(s), or {} byte(s), of input", input.len(), byte_count);
    Ok(())
}

pub fn apply_string_multi<'e, E, R, W>(exprs: E, input: R, mut output: &mut W) -> io::Result<()>
    where E: Iterator<Item=&'e str>, R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    let mut reader = BufReader::new(input);
    let mut input = String::new();
    let byte_count = try!(reader.read_to_string(&mut input));

    let mut context = Context::new();
    update_context(&mut context, &input);

    for ast in asts {
        let result = {
            let value = context.get("_").unwrap();
            try!(evaluate(&ast, value, &context))
        };

        // TODO(xion): really consider deprecating the _X conversion placeholders,
        // as this is another reason why they are cumbersome (in addition to dubious FromStr
        // impl on Value); _ would then ALWAYS be a string
        // context.unset_here("_b");
        // context.unset_here("_f");
        // context.unset_here("_i");
        // context.unset_here("_s");
        // XXX: can't actually do the above because of `value` borrowing `context` immutably -_-

        context.set("_", result);
    }
    let result = context.get("_").unwrap();
    try!(write_result(&mut output, result));

    info!("Processed {} character(s), or {} byte(s), through {} expression(s)",
          input.len(), byte_count, expr_count);
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
    try!(write_result(&mut writer, &result));

    info!("Processed {} line(s) of input", count);
    Ok(())
}


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

        let value = context.get("_").unwrap();
        let result = try!(evaluate(&ast, value, &context));
        try!(write_result(&mut writer, &result));

        count += 1;
    }

    info!("Processed {} line(s) of input", count);
    Ok(())
}

/// Apply the expression to given input stream, word by word,
/// (each word treated as string in the expression itself),
/// and writing to the given output stream.
pub fn map_words<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    let mut count = 0;
    {
        let mut maybe_process_word = |w: &mut String| -> io::Result<()> {
            if w.is_empty() {
                return Ok(());
            }
            context.set("_", Value::String(w.clone()));
            let value = context.get("_").unwrap();

            // TODO(xion): preserve the exact sequences of whitespace between words
            let result = try!(evaluate(&ast, value, &context));
            let retval = try!(String::try_from(result)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
            try!(write!(writer, "{} ", retval));

            count += 1;
            w.clear();
            Ok(())
        };

        let mut word = String::new();
        for line in reader.lines() {
            let line = try!(line);
            for ch in line.chars() {
                if ch.is_whitespace() {
                    try!(maybe_process_word(&mut word));
                } else {
                    word.push(ch);
                }
            }
            try!(maybe_process_word(&mut word));
        }
    }

    info!("Processed {} word(s) of input", count);
    Ok(())
}

/// Apply the expression to given input stream, character by character
/// (treated as 1-character string in the expression itself),
/// and writing to the given output stream.
pub fn map_chars<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    let mut count = 0;
    {
        let mut process_char = |ch: char| -> io::Result<()> {
            context.set("_", Value::from(ch));
            let value = context.get("_").unwrap();

            let result = try!(evaluate(&ast, value, &context));
            try!(write_result(&mut writer, &result));

            count += 1;
            Ok(())
        };

        // TODO(xion): rather than reading the input line by line,
        // use Read::chars() when the feature is stable (same in map_words)
        for line in reader.lines() {
            let line = try!(line);
            for ch in line.chars() {
                try!(process_char(ch));
            }
            // TODO(xion): cross-platfrorm line ending
            try!(process_char('\n'));
        }
    }

    info!("Processed {} character(s) of input", count);
    Ok(())
}

/// Apply the expression to bytes of given input stream,
/// writing the transformed bytes into given output stream.
///
/// Note that the expression must always produce a byte (i.e. an integer from the 0-255 range).
pub fn map_bytes<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    let ast = try!(parse_expr(expr));

    // we will be handling individual bytes, but buffering can still be helpful
    // if the underlying reader/writer is something slow like a disk or network
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    let mut count = 0;
    for byte in reader.bytes() {
        let byte = try!(byte);
        context.set("_", Value::from(byte));
        let value = context.get("_").unwrap();

        let result = try!(evaluate(&ast, value, &context));
        match result {
            Value::Integer(i) if 0 <= i && i < u8::MAX as IntegerRepr => {
                try!(writer.write_all(&[i as u8]))
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("expected a byte-sized integer, got {}", result))),
        }

        count += 1;
    }

    info!("Processed {} byte(s) of input", count);
    Ok(())
}

// Utility functions.

fn parse_expr(expr: &str) -> io::Result<Box<Eval>> {
    parse(expr).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn parse_exprs<'e, E>(exprs: E) -> io::Result<Vec<Box<Eval>>>
    where E: Iterator<Item=&'e str>
{
    let mut result = Vec::new();
    for expr in exprs {
        debug!("Parsing expression: {}", expr);
        let ast = try!(parse_expr(expr));
        result.push(ast);
    }
    Ok(result)
}

fn update_context(context: &mut Context, input: &str) {
    context.set("_", input.parse::<Value>().unwrap_or_else(|_| Value::String(input.to_owned())));
    context.set("_b", input.parse::<BooleanRepr>().map(Value::Boolean).unwrap_or(Value::Empty));
    context.set("_f", input.parse::<FloatRepr>().map(Value::Float).unwrap_or(Value::Empty));
    // TODO(xion): consider also trying to parse the input as f64
    // and exposing the rounded version as _i
    context.set("_i", input.parse::<IntegerRepr>().map(Value::Integer).unwrap_or(Value::Empty));
    context.set("_s", Value::String(input.to_owned()));
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

fn write_result<W: Write>(output: &mut W, result: &Value) -> io::Result<()> {
    let result = try!(String::try_from(result)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
    write!(output, "{}\n", result)
}
