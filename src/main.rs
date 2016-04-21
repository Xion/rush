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

use rush::Context;

use args::InputMode;


fn main() {
    logging::init().unwrap();

    let opts = args::parse();
    let exprs: Vec<&str> = opts.expressions.iter().map(|e| e as &str).collect();

    match opts.input_mode {
        Some(mode) => {
            if let Err(error) = process_input(mode, &exprs) {
                error!("{:?}", error);
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
fn process_input(mode: InputMode, exprs: &[&str]) -> io::Result<()> {
    let apply_multi_ctx: fn(&mut Context, _, _, _) -> _ = match mode {
        InputMode::String => rush::apply_string_multi_ctx,
        InputMode::Lines => rush::map_lines_multi_ctx,
        InputMode::Words => rush::map_words_multi_ctx,
        InputMode::Chars => rush::map_chars_multi_ctx,
        InputMode::Bytes => rush::map_bytes_multi_ctx,
    };

    let mut context = Context::new();
    apply_multi_ctx(&mut context, &exprs, io::stdin(), &mut io::stdout())
}


/// Print the AST for given expression to stdout.
fn print_ast(expr: &str) {
    debug!("Printing the AST of:  {}", expr);
    match rush::parse(expr) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => { error!("{:?}", error); exit(1); },
    }
}
