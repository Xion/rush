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
    let apply_multi_ctx: fn(&mut Context, _, _, _) -> _ = match mode {
        InputMode::String => rush::apply_string_multi_ctx,
        InputMode::Lines => rush::map_lines_multi_ctx,
        InputMode::Words => rush::map_words_multi_ctx,
        InputMode::Chars => rush::map_chars_multi_ctx,
        InputMode::Bytes => rush::map_bytes_multi_ctx,
    };

    let mut context = Context::new();
    if let Some(before) = before {
        try!(rush::exec(before, &mut context));
    }
    if let Some(_) = after {
        unimplemented!();
    }

    apply_multi_ctx(&mut context, &exprs, io::stdin(), &mut io::stdout())
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
