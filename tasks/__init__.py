"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
import logging
import os
import sys

from invoke import Collection, task


BIN = 'rush'
LIB = 'librush'


@task(default=True)
def default(ctx):
    """Run all the tests & lints."""
    test.all(ctx)
    lint.all(ctx)


# Task setup

ns = Collection()
ns.add_task(default)

from tasks import build, clean, lint, release, run, test
ns.add_collection(build)
ns.add_collection(clean)
ns.add_collection(lint)
ns.add_collection(run)
ns.add_collection(release)
ns.add_collection(test)

from tasks import misc
ns.add_task(misc.print_grammar)


# This precondition makes it easier to localize various auxiliary files
# that the tasks need, like Cargo.toml's of crates, etc.
if not os.path.exists(os.path.join(os.getcwd(), '.gitignore')):
    logging.fatal(
        "Automation tasks can only be invoked from project's root directory!")
    sys.exit(1)
