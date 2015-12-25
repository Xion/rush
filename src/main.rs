extern crate getopts;
#[macro_use]
extern crate nom;

mod ast;
mod eval;
mod parse;


use std::env;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

use getopts::Options;

use self::eval::{Eval, Context};
use self::parse::parse;


fn main() {
    let argv: Vec<String> = env::args().collect();
    let program = argv[0].clone();

    let mut options = Options::new();
    options.optflag("h", "help", "Show this usage message");

    let args = options.parse(&argv[1..]).unwrap();
    if args.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    let expr = args.free.join(" ");
    apply(&expr, io::stdin(), io::stdout());
}


/// Print the instructions about invoking the program from the command line.
fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}


/// Apply the expression to given input stream,
/// writing to the given output stream.
fn apply<R: Read, W: Write>(expr: &str, input: R, output: W) {
    let ast = parse(expr);

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    for line in reader.lines() {
        // TODO(xion): handle read errors
        let line = line.unwrap();

        let mut context = Context::new();
        context.insert("_".to_string(), line);

        let result = ast.eval(&context);

        // TODO(xion): handle write errors
        write!(writer, "{}\n", result).unwrap();
    }
}
