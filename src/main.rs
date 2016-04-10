//! Module with the entry point of the binary.

extern crate case;
extern crate clap;
extern crate conv;
#[macro_use]
extern crate log;

extern crate rush;


mod args;
mod logging;


use std::io;
use std::process::exit;

use args::InputMode;


fn main() {
    logging::init().unwrap();

    let opts = args::parse();
    let exprs: Vec<&str> = opts.expressions.iter().map(|e| e as &str).collect();

    if opts.input_mode.is_none() {
        for expr in exprs {
            print_ast(expr);
        }
        return;
    }

    // choose a function to process the input with, depending on flags
    let apply_multi: fn(_, _, _) -> _ = match opts.input_mode.unwrap() {
        InputMode::String => rush::apply_string_multi,
        InputMode::Lines => rush::map_lines_multi,
        InputMode::Words => rush::map_words_multi,
        InputMode::Chars => rush::map_chars_multi,
        InputMode::Bytes => rush::map_bytes_multi,
    };
    if let Err(error) = apply_multi(&exprs, io::stdin(), &mut io::stdout()) {
        error!("{:?}", error);
        exit(1);
    }
}


/// Print the AST for given expression to stdout.
fn print_ast(expr: &str) {
    debug!("Printing the AST of:  {}", expr);
    match rush::parse(expr) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => { error!("{:?}", error); exit(1); },
    }
}
