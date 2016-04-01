//! Module with the entry point of the binary.

extern crate clap;
#[macro_use]
extern crate log;

extern crate rush;


mod args;
mod logging;


use std::io;
use std::process::exit;


fn main() {
    logging::init().unwrap();

    let args = args::parse();
    let expr = args.value_of("expr").unwrap();

    if args.is_present("parse") {
        print_ast(expr);
        return;
    }

    // choose a function to process the input with, depending on flags
    let input_mode = |mode| {
        args.value_of("mode") == Some(mode) || args.is_present(mode)
    };
    let apply: fn(_, _, _) -> _ =
        if input_mode("string") {
            rush::apply_string
        } else if input_mode("lines") {
            rush::map_lines
       } else {
            info!("Using default processing mode (line-by-line)");
            rush::map_lines
        };
    if let Err(error) = apply(expr, io::stdin(), &mut io::stdout()) {
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
