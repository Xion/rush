//! Module for handling command line arugments to the application.

use std::env;
use std::ffi::OsString;
use std::iter::IntoIterator;

use clap::{self, AppSettings, Arg, ArgSettings, ArgGroup, ArgMatches};


/// Parse command line arguments and return matches' object.
pub fn parse<'a>() -> ArgMatches<'a> {
    parse_from_argv(env::args_os())
}

/// Parse application arguments given array of arguments
/// (*all* arguments, including binary name).
pub fn parse_from_argv<'a, I, T>(argv: I) -> ArgMatches<'a>
    where I: IntoIterator<Item=T>, T: Into<OsString>
{
    create_parser().get_matches_from(argv)
}


/// Creates the argument parser
/// (which is called an "App" in clap's silly nomenclature).
fn create_parser<'a>() -> clap::App<'a, 'a> {
    let mut parser = clap::App::new("ap");
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        parser = parser.version(version);
    }
    parser
        .about("sed/awk for Humans (tm)")
        .author("Karol Kuczmarski")

        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)

        .group(ArgGroup::with_name("action")
            .args(&["all", "lines", "words"]))
        .arg(Arg::with_name("all")
            .short("a").long("all")
            .help("Treat input as array of lines to apply the expression to"))
        .arg(Arg::with_name("lines")
            .short("l").long("lines")
            .help("Apply the expression to each line of input as string. This is the default"))
        .arg(Arg::with_name("words")
            .short("w").long("words")
            .help("Apply the expression to each line of input as array of words (NYI)"))
        .arg(Arg::with_name("json")
            .short("j").long("json")
            .help("Apply the expression to the whole output interpreted as JSON (NYI)"))

        .arg(Arg::with_name("parse")
            .set(ArgSettings::Hidden)
            .conflicts_with("action")
            .short("p").long("parse")
            .help("Only parse the expression, printing its AST"))

        .arg(Arg::with_name("expr")
            .use_delimiter(false)  // don't interpret comma as arg separator
            .help("Expression to apply to input")
            .required(true))
}
