"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
from __future__ import print_function

from glob import glob
import logging
import os
import sys

from invoke import Collection, task


BIN = 'rush'
LIB = 'librush'


@task(name="print-grammar")
def print_grammar(ctx):
    """Prints the language's grammar rules.
    Uses comments in the syntax definition & parsing code.
    """
    syntax_package = './crates/%s/src/parse/syntax' % (LIB,)
    for filename in glob('%s/*.rs' % syntax_package):
        with open(filename) as f:
            for line in f.readlines():
                # TODO: support grammar rules spanning more than one line
                if '::==' in line:
                    line = line.lstrip('/').strip()
                    print(line)


# Task setup

ns = Collection()
ns.add_task(print_grammar)

from tasks import build, clean, release, run, test
ns.add_collection(build)
ns.add_collection(clean)
ns.add_collection(run)
ns.add_collection(release)
ns.add_collection(test)

ns.add_task(test.all, name='default', default=True)


# This precondition makes it easier to localize various auxiliary files
# that the tasks need, like Cargo.toml's of crates, etc.
if not os.path.exists(os.path.join(os.getcwd(), '.gitignore')):
    logging.fatal(
        "Automation tasks can only be invoked from project's root directory!")
    sys.exit(1)
