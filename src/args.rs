//! Module for handling command line arguments to the application.

use std::env;
use std::ffi::OsString;
use std::iter::IntoIterator;

use clap::{self, AppSettings, Arg, ArgSettings, ArgGroup, ArgMatches};


/// Defines possible options as to how the program's input
/// may be processed by the expression.
#[derive(Clone)]
pub enum InputMode {
    String,
    Lines,
    Chars,
}

impl InputMode {
    fn description(&self) -> &str {
        match *self {
            InputMode::String => "whole input as string",
            InputMode::Lines => "line by line",
            InputMode::Chars => "character by character",
        }
    }
}

impl Default for InputMode {
    fn default() -> Self { InputMode::Lines }
}

/// Structure holding the options parsed from command line.
#[derive(Clone)]
pub struct Options {
    pub expression: String,
    pub input_mode: Option<InputMode>,
}

impl<'a> From<ArgMatches<'a>> for Options {
    fn from(matches: ArgMatches<'a>) -> Self {
        let input_mode = if matches.is_present(OPT_PARSE) {
            None
        } else {
            // decide the input mode based either on --input flag's value
            // or dedicated flags, like --string
            let mode_is = |mode| {
                matches.value_of(OPT_INPUT_MODE) == Some(mode) || matches.is_present(mode)
            };
            Some(
                if mode_is("string")        { InputMode::String }
                else if mode_is("lines")    { InputMode::Lines }
                else if mode_is("chars")    { InputMode::Chars }
                else {
                    let default = InputMode::default();
                    info!("Using default processing mode ({})", default.description());
                    default
                }
            )
        };
        Options{
            expression: matches.value_of(ARG_EXPRESSION).unwrap().to_owned(),
            input_mode: input_mode,
        }
    }
}


/// Parse command line arguments and return matches' object.
#[inline(always)]
pub fn parse() -> Options {
    parse_from_argv(env::args_os())
}

/// Parse application arguments given array of arguments
/// (*all* arguments, including binary name).
#[inline(always)]
pub fn parse_from_argv<I, T>(argv: I) -> Options
    where I: IntoIterator<Item=T>, T: Into<OsString>
{
    let matches = create_parser().get_matches_from(argv);
    Options::from(matches)
}


// Parser configuration

// Type of the argument parser object.
type Parser<'p> = clap::App<'p, 'p>;


const APP_NAME: &'static str = "rush";
const APP_DESC: &'static str = "Succint & readable processing language";
const APP_AUTHOR: &'static str = "Karol Kuczmarski";

const USAGE: &'static str = "rush [--input <MODE> | --string | --lines | --chars] <EXPRESSION>";

const ARG_EXPRESSION: &'static str = "expr";
const OPT_INPUT_MODE: &'static str = "mode";
const INPUT_MODES: &'static [&'static str] = &["string", "lines", "chars"];
const OPT_PARSE: &'static str = "parse";


/// Creates the argument parser
/// (which is called an "App" in clap's silly nomenclature).
fn create_parser<'p>() -> Parser<'p> {
    let mut parser = clap::App::new(APP_NAME);
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        parser = parser.version(version);
    }
    parser
        .about(APP_DESC)
        .author(APP_AUTHOR)
        .usage(USAGE)

        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DeriveDisplayOrder)

        // TODO(xion): consider implementing more input modes:
        // * words - each word evaluated separately
        // * bytes - each byte separately (as integer)
        .group(ArgGroup::with_name("input_group")
            .arg(OPT_INPUT_MODE)
            .args(INPUT_MODES))
        .arg(Arg::with_name(OPT_INPUT_MODE)
            .short("i").long("input")
            .takes_value(true)
            .possible_values(INPUT_MODES)
            .help("Defines how the input should be treated when processed by EXPRESSION")
            .value_name("MODE")
            .set(ArgSettings::NextLineHelp))
        .arg(Arg::with_name("string")
            .short("s").long("string")
            .help("Apply the expression once to the whole input as single string"))
        .arg(Arg::with_name("lines")
            .short("l").long("lines")
            .help("Apply the expression to each line of input as string. This is the default"))
        .arg(Arg::with_name("chars")
            .short("c").long("chars")
            .help("Apply the expression to each character of input (which is treated as 1-character string)."))

        .arg(Arg::with_name(OPT_PARSE)
            .set(ArgSettings::Hidden)
            .conflicts_with("input_group")
            .short("p").long("parse")
            .help("Only parse the expression, printing its AST"))

        .arg(Arg::with_name(ARG_EXPRESSION)
            .use_delimiter(false)  // don't interpret comma as arg separator
            .help("Expression to apply to input")
            .value_name("EXPRESSION")
            .required(true))

        .help_short("H")
        .version_short("V")
}
