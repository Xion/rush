//! Module for handling of the .Xrc files.
//!
//! Those files -- which can be in various places in the system but mostly inside
//! the home directory -- may define expressions to be executed right before
//! processing the input.
//!
//! The expressions will be executed within the root Context that's reused between
//! all expressions evaluated during an invocation of the binary.
//! The primary application of this is to define additional functions & other symbols
//! to be available to all expressions.

use std::env;
use std::convert::AsRef;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use rush::{self, Context};


/// Stems of the .Xrc filenames, of files that we'll load the symbols from.
/// Note they shouldn't have the leading dot nor the actual "rc" suffix.
const STEMS: &'static [&'static str] = &["rush", "rh"];

/// Prefix of comment lines.
const COMMENT_PREFIX: &'static str = "//";


/// Load the definitions from all available .Xrc files into given Context.
pub fn load_into(context: &mut Context) -> io::Result<()> {
    for path in list_rcfiles() {
        debug!("Loading symbols from {}", path.display());
        let file = try!(File::open(&path));
        let content = try!(read_rcfile(file));
        try!(rush::exec(&content, context));
        info!("Symbols loaded from {}", path.display());
    }
    Ok(())
}


/// Read an .Xrc file, discarding all the comments & empty lines.
fn read_rcfile<R: Read>(file: R) -> io::Result<String> {
    let mut result = String::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = try!(line);
        if line.trim().is_empty() || line.trim().starts_with(COMMENT_PREFIX) {
            continue;
        }
        result.push_str(&line);
    }
    Ok(result)
}


/// List the full paths to all .Xrc files in the system,
/// in the order they should be read.
fn list_rcfiles() -> Vec<PathBuf> {
    // List directories eligible for having their .Xrc files read.
    // Note that the order matters: directories with higher priority should be
    // considered last, so that definitions inside their .Xrc files can override
    // the ones that come before them.
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(homedir) = env::home_dir() {
        dirs.push(homedir);
    }
    match env::current_dir() {
        Ok(cwd) => dirs.push(cwd.to_owned()),
        Err(err) => warn!("Couldn't retrieve current directory: {}", err),
    }

    // Return those .Xrc files that actually exist.
    let mut result = Vec::new();
    for dir in dirs.into_iter().map(PathBuf::from) {
        for name in rc_filenames() {
            let path = dir.join(name);
            if file_exists(&path) {
                result.push(path);
            }
        }
    }
    result
}

/// Get the possible names of .Xrc files within any directory.
fn rc_filenames() -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();
    for stem in STEMS {
        result.push(PathBuf::from(format!(".{}rc", stem)));
    }
    for stem in STEMS {
        result.push(PathBuf::from(".config").join(format!(".{}rc", stem)));
    }
    result
}

// Utility functions

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(&path) {
        Ok(_) => true,
        Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
                warn!("Couldn't determine if {} exists: {}", path.as_ref().display(), err);
            }
            false
        }
    }
}

#[allow(dead_code)]
fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(&path) {
        Ok(metadata) => metadata.is_dir(),
        Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
                warn!("Couldn't determine if {} exists: {}", path.as_ref().display(), err);
            }
            false
        },
    }
}
