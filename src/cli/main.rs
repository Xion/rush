//! Module with the entry point of the binary.

extern crate case;
extern crate clap;
extern crate conv;
#[macro_use]
extern crate log;

extern crate rush;


mod args;
mod logging;


use std::error::Error;  // for .cause() method
use std::io::{self, Write};
use std::iter::repeat;
use std::process::exit;

use conv::TryFrom;
use rush::Context;

use args::InputMode;


fn main() {
    logging::init().unwrap();

    let opts = args::parse();

    let before = opts.before.as_ref().map(|b| b as &str);
    let exprs: Vec<&str> = opts.expressions.iter().map(|e| e as &str).collect();
    let after = opts.after.as_ref().map(|a| a as &str);

    match opts.input_mode {
        Some(mode) => {
            if let Err(error) = process_input(mode, before, &exprs, after) {
                handle_error(error);
                exit(1);
            }
        },
        None => {
            for expr in exprs {
                print_ast(expr);
            }
        },
    }
}


/// Process standard input through given expressions, writing results to stdout.
fn process_input(mode: InputMode,
                 before: Option<&str>, exprs: &[&str], after: Option<&str>) -> io::Result<()> {
    // Prepare a Context for the processing.
    // This includes evaluating any "before" expression within it.
    let mut context = Context::new();
    if let Some(before) = before {
        try!(rush::exec(before, &mut context));
    }

    // Do the processing.
    //
    // If there is an "after" expression provided, it is that expression that should produced
    // the only output of the program. So we'll just consume whatever results would normally
    // be printed otherwise.
    let mut stdout = io::stdout();      // Those intermediate bindings are necessary
    let mut sink = io::sink();          // as Rust doesn't have named lifetime scopes yet.
    let mut output: &mut Write = if after.is_some() { &mut sink } else { &mut stdout };
    try!(apply_multi_ctx(mode, &mut context, exprs, &mut output));

    // Evaluate the "after" expression, if provided, and return it as the result.
    if let Some(after) = after {
        let result = try!(rush::eval(after, &mut context));
        let result_string = try!(String::try_from(result)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
        // TODO(xion): omit \n for multi-line results
        return write!(&mut io::stdout(), "{}\n", result_string);
    }

    Ok(())
}

/// Apply the expressions to the standard input with given mode.
/// This forms the bulk of the input processing.
#[inline]
fn apply_multi_ctx(mode: InputMode,
                   context: &mut Context, exprs: &[&str], mut output: &mut Write) -> io::Result<()> {
    let func: fn(_, _, _, _) -> _ = match mode {
        InputMode::String => rush::apply_string_multi_ctx,
        InputMode::Lines => rush::map_lines_multi_ctx,
        InputMode::Words => rush::map_words_multi_ctx,
        InputMode::Chars => rush::map_chars_multi_ctx,
        InputMode::Bytes => rush::map_bytes_multi_ctx,
    };
    func(context, exprs, io::stdin(), &mut output)
}

/// Handle an error that occurred while processing the input.
fn handle_error(error: io::Error) {
    writeln!(&mut io::stderr(), "error: {}", error).unwrap();

    // Print the error causes as an indented "tree".
    let mut cause = error.cause();
    let mut indent = 0;
    while let Some(error) = cause {
        writeln!(&mut io::stderr(), "{}{}{}",
            repeat(" ").take(CAUSE_PREFIX.len() * indent).collect::<String>(),
            CAUSE_PREFIX,
            error).unwrap();
        indent += 1;
        cause = error.cause();
    }
}

const CAUSE_PREFIX: &'static str = "└ ";  // U+2514


/// Print the AST for given expression to stdout.
fn print_ast(expr: &str) {
    debug!("Printing the AST of:  {}", expr);
    match rush::parse(expr) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => { error!("{:?}", error); exit(1); },
    }
}
