//! Module with the entry point of the binary.

extern crate case;
extern crate clap;
extern crate conv;
#[macro_use]
extern crate log;

extern crate rush;


mod args;
mod logging;
mod rcfile;


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
            if let Some(before) = before {
                println!("--before expression:");
                print_ast(before);
                println!("");
            }
            for expr in exprs {
                print_ast(expr);
            }
            if let Some(after) = after {
                println!("");
                println!("--after expression:");
                print_ast(after);
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
    try!(rcfile::load_into(&mut context));
    if let Some(before) = before {
        try!(rush::exec(before, &mut context));
    }

    // Do the processing.
    //
    // If there is an "after" expression provided, it is that expression that should produced
    // the only output of the program. So we'll just consume whatever results would normally
    // be printed otherwise.
    if after.is_some() {
        // HACK: Because the intermediate results have to be printed out -- even if only to /dev/null
        // -- we have to ensure there is always a non-empty value to use as the intermediate result.
        // This is necessary especially since with --after (and --before), intermediate expressions
        // are likely to be just assignments (and the result of an assignment is empty).
        //
        // We can make sure there is always a value to print simply by adding one more expression
        // at the end of the chain. It so happens that zero (or any number) is compatible with
        // all the input modes, so let's use that.
        let mut exprs = exprs.to_vec();
        exprs.push("0");
        try!(apply_multi_ctx(mode, &mut context, &exprs, &mut io::sink()));
    } else {
        try!(apply_multi_ctx(mode, &mut context, exprs, &mut io::stdout()));
    }

    // Evaluate the "after" expression, if provided, and return it as the result.
    if let Some(after) = after {
        let result = try!(rush::eval(after, &mut context));
        let result_string = try!(String::try_from(result)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));

        // Make it so that the output always ends with a newline,
        // regardless whether it consists of a single value or multiple lines.
        return if result_string.ends_with("\n") {
            write!(&mut io::stdout(), "{}", result_string)
        } else {
            write!(&mut io::stdout(), "{}\n", result_string)
        };
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
        InputMode::Files => rush::map_files_multi_ctx,
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
