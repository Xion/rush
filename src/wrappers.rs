//! Convenience wrappers around parsing and evaluation.

use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::u8;

use conv::TryFrom;

use super::eval::{self, Eval, Context, Invoke, Value};
use super::eval::value::IntegerRepr;
use super::parse::parse;


/// Name of the variable within expression context that holds the current/input value.
const CURRENT: &'static str = "_";


// Single-expression processing.

/// Apply the expresion to a complete input stream, processed as single string,
/// writing to the given output stream.
#[inline(always)]
pub fn apply_string<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    apply_string_multi(&[expr], input, output)
}

/// Apply the expression to given input taken as array of lines,
/// writing result to the given output stream.
#[inline(always)]
pub fn apply_lines<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    apply_lines_multi(&[expr], input, output)
}

/// Apply the expression to given input stream, line by line,
/// writing to the given output stream.
#[inline(always)]
pub fn map_lines<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    map_lines_multi(&[expr], input, output)
}

/// Apply the expression to given input stream, word by word,
/// (each word treated as string in the expression itself),
/// and writing to the given output stream.
#[inline(always)]
pub fn map_words<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    map_words_multi(&[expr], input, output)
}

/// Apply the expression to given input stream, character by character
/// (treated as 1-character string in the expression itself),
/// and writing to the given output stream.
pub fn map_chars<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    map_chars_multi(&[expr], input, output)
}

/// Apply the expression to bytes of given input stream,
/// writing the transformed bytes into given output stream.
///
/// Note that the expression must always produce a byte (i.e. an integer from the 0-255 range).
pub fn map_bytes<R: Read, W: Write>(expr: &str, input: R, output: &mut W) -> io::Result<()> {
    map_bytes_multi(&[expr], input, output)
}


// Multi-expression processing.

/// Apply a sequence of expressions to the input stream taken as single string.
///
/// The stream is provided as a single string to the first expression,
/// whose result is then passed to the second one, etc.
///
/// The final result is written to the given output stream.
#[inline(always)]
pub fn apply_string_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    apply_string_multi_ctx(&mut context, exprs, input, output)
}

/// Apply a sequence of expressions to the input stream taken as an array of lines
///
/// The stream is provided as an array of strings to the first expression,
/// whose result is then passed to the second one, etc.
///
/// The final result is written to the given output stream.
pub fn apply_lines_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    apply_lines_multi_ctx(&mut context, exprs, input, output)
}

/// Apply a sequence of expressions to the input stream, line by line.
///
/// Every line read from the stream is fed to the first expression
/// (without the \n char) whose result is then passed to the second one, etc.
///
/// The final result is written then to the given output stream.
/// This continues for each line of input.
pub fn map_lines_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    map_lines_multi_ctx(&mut context, exprs, input, output)
}

/// Apply a sequence of expressions to the input stream, word by word.
///
/// Every word read from the stream is fed to the first expression,
/// whose result is then passed to the second one, etc.
///
/// The final result is written then to the given output stream.
/// This continues for each word of input.
pub fn map_words_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    map_words_multi_ctx(&mut context, exprs, input, output)
}

/// Apply a sequence of expressions to the input stream, character by character.
///
/// Every character read from the stream is fed to the first expression
/// (as 1-character string), whose result is then passed to the second one, etc.
///
/// The final result is written then to the given output stream.
/// This continues for each character of input.
pub fn map_chars_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    map_chars_multi_ctx(&mut context, exprs, input, output)
}

/// Apply a sequence of expressions to the input stream, byte by byte.
///
/// Every byte read from the stream is fed to the first expression
/// (as an integer from 0-255 range), whose result is then passed to the second one, etc.
///
/// The final result -- which has to be a 0-255 integer -- is written then
/// to the given output stream. This continues for each byte of input.
pub fn map_bytes_multi<R: Read, W: Write>(exprs: &[&str], input: R, output: &mut W) -> io::Result<()> {
    let mut context = Context::new();
    map_bytes_multi_ctx(&mut context, exprs, input, output)
}

// Multi-expression processing with shared context.

/// Apply a sequence of expressions to the input stream taken as single string.
///
/// The stream is provided as a single string to the first expression,
/// whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result is written to the given output stream.
pub fn apply_string_multi_ctx<R, W>(context: &mut Context,
                                    exprs: &[&str],
                                    input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    let mut reader = BufReader::new(input);
    let mut input = String::new();
    let byte_count = try!(reader.read_to_string(&mut input));
    let char_count = input.chars().count();

    context.set(CURRENT, Value::String(input));

    let result = try!(process(context, &asts));
    try!(write_result_line(output, result));

    info!("Processed {} character(s), or {} byte(s), through {} expression(s)",
          char_count, byte_count, expr_count);
    Ok(())
}

/// Apply a sequence of expressions to the input stream taken as an array of lines.
///
/// The stream is provided as an array of strings to the first expression,
/// whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result is written to the given output stream.
pub fn apply_lines_multi_ctx<R, W>(context: &mut Context,
                                   exprs: &[&str],
                                   input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    // parse input lines into a vector of Value objects
    let lines: Vec<_> = BufReader::new(input).lines()
        .map(|r| {
            r.ok().expect("failed to read input line")
                .parse::<Value>().unwrap_or(Value::Empty)
        })
        .filter(|v| *v != Value::Empty)
        .collect();
    let line_count = lines.len();

    context.set(CURRENT, Value::Array(lines));

    let result = try!(process(context, &asts));
    try!(write_result_line(output, result));

    info!("Processed {} line(s) of input through {} expression(s)",
          line_count, expr_count);
    Ok(())
}

/// Apply a sequence of expressions to the input stream, line by line.
///
/// Every line read from the stream is fed to the first expression
/// (without the \n char) whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result is written then to the given output stream.
/// This continues for each line of input.
pub fn map_lines_multi_ctx<R, W>(context: &mut Context,
                                 exprs: &[&str],
                                 input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut line_count = 0;
    for line in reader.lines() {
        let line = try!(line);
        context.set(CURRENT, to_value(line));

        let result = try!(process(context, &asts));
        try!(write_result_line(&mut writer, result));

        line_count += 1;
    }

    info!("Processed {} line(s) of input through {} expression(s)",
          expr_count, line_count);
    Ok(())
}

/// Apply a sequence of expressions to the input stream, word by word.
///
/// Every word read from the stream is fed to the first expression,
/// whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result is written then to the given output stream.
/// This continues for each word of input.
pub fn map_words_multi_ctx<R, W>(context: &mut Context,
                                 exprs: &[&str],
                                 input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut word_count = 0;
    {
        // Note that `writer` is taken as a parameter rather than just being
        // captured by the closure, because we need to refer to it mutably
        // in the main loop below.
        let mut maybe_process_word = |word: &mut String,
                                      writer: &mut Write| -> io::Result<()> {
            if word.is_empty() {
                return Ok(());
            }

            context.set(CURRENT, to_value(word.clone()));
            let result = try!(process(context, &asts));

            let retval = try!(String::try_from(result)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
            try!(write!(writer, "{}", retval));

            word_count += 1;
            word.clear();
            Ok(())
        };

        let mut word = String::new();
        for line in reader.lines() {
            let line = try!(line);
            for ch in line.chars() {
                // Whitespace characters denote word's end, but they are to be
                // preserved verbatim in the final output.
                if ch.is_whitespace() {
                    try!(maybe_process_word(&mut word, &mut writer));
                    try!(write!(writer, "{}", ch));
                } else {
                    word.push(ch);
                }
            }
            try!(maybe_process_word(&mut word, &mut writer));
        }
    }

    info!("Processed {} word(s) of input through {} expression(s)",
          word_count, expr_count);
    Ok(())
}

/// Apply a sequence of expressions to the input stream, character by character.
///
/// Every character read from the stream is fed to the first expression
/// (as 1-character string), whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result is written then to the given output stream.
/// This continues for each character of input.
pub fn map_chars_multi_ctx<R, W>(context: &mut Context,
                                 exprs: &[&str],
                                 input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut char_count = 0;
    {
        let mut process_char = |ch: char| -> io::Result<()> {
            context.set(CURRENT, Value::from(ch));

            // TODO(xion): consider enforcing for the final result to also be 1-char string
            // and writing those characters as a contiguous string
            let result = try!(process(context, &asts));
            try!(write_result_line(&mut writer, &result));

            char_count += 1;
            Ok(())
        };

        // TODO(xion): rather than reading the input line by line,
        // use Read::chars() when the feature is stable (same in map_words_multi_ctx)
        for line in reader.lines() {
            let line = try!(line);
            for ch in line.chars() {
                try!(process_char(ch));
            }
            // TODO(xion): cross-platfrorm line ending
            try!(process_char('\n'));
        }
    }

    info!("Processed {} character(s) of input through {} expression(s)",
          char_count, expr_count);
    Ok(())
}

/// Apply a sequence of expressions to the input stream, byte by byte.
///
/// Every byte read from the stream is fed to the first expression
/// (as an integer from 0-255 range), whose result is then passed to the second one, etc.
/// Expression context is shared throughout.
///
/// The final result -- which has to be a 0-255 integer -- is written then
/// to the given output stream. This continues for each byte of input.
pub fn map_bytes_multi_ctx<R, W>(context: &mut Context,
                                 exprs: &[&str],
                                 input: R, output: &mut W) -> io::Result<()>
    where R: Read, W: Write
{
    let asts = try!(parse_exprs(exprs));
    let expr_count = asts.len();

    // we will be handling individual bytes, but buffering can still be helpful
    // if the underlying reader/writer is something slow like a disk or network
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut byte_count = 0;
    for byte in reader.bytes() {
        let byte = try!(byte);
        context.set(CURRENT, Value::from(byte));

        let result = try!(process(context, &asts));
        match *result {
            Value::Integer(i) if 0 <= i && i < u8::MAX as IntegerRepr => {
                try!(writer.write_all(&[i as u8]))
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("expected a byte-sized integer, got {}", result))),
        }

        byte_count += 1;
    }

    info!("Processed {} byte(s) of input through {} expression(s)",
          byte_count, expr_count);
    Ok(())
}

// Utility functions.

fn parse_exprs(exprs: &[&str]) -> io::Result<Vec<Box<Eval>>> {
    let mut result = Vec::new();
    for expr in exprs {
        debug!("Parsing expression: {}", expr);
        let ast = try!(parse(expr)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)));
        result.push(ast);
    }
    Ok(result)
}

fn to_value(input: String) -> Value {
    input.parse::<Value>().unwrap_or_else(|_| Value::String(input))
}

fn process<'c>(context: &'c mut Context, exprs: &[Box<Eval>]) -> io::Result<&'c Value> {
    for ast in exprs.iter() {
        let result = {
            let value = context.get(CURRENT).unwrap();
            try!(evaluate(ast, value, context))
        };
        context.set(CURRENT, result);
    }
    Ok(context.get(CURRENT).unwrap())
}

fn evaluate<'c>(ast: &Box<Eval>, input: &'c Value, context: &'c Context) -> io::Result<Value> {
    ast.eval(context)
        .and_then(|result| maybe_apply_result(result, input, &context))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn maybe_apply_result<'c>(result: Value, input: &'c Value, context: &'c Context) -> eval::Result {
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

fn write_result_line<W: Write>(output: &mut W, result: &Value) -> io::Result<()> {
    let result = try!(String::try_from(result)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
    write!(output, "{}\n", result)
}
