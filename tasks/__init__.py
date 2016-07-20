"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
import logging
import os
import sys

from invoke import Collection


BIN = 'rush'
LIB = 'librush'


from tasks import build, clean, release, run, test

ns = Collection()
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
