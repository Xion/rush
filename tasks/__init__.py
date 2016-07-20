"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
from invoke import Collection, task


BIN = 'rush'
LIB = 'librush'


# TODO: rewrite the ./tools/release script in Python into nested tasks
# like release.deb or release.rpm or release.brew
@task
def release():
    """Create the release packages for various operating systems."""
    from invoke import run
    run('./tools/release', pty=True)


# Setup

ns = Collection(release)

from tasks import build, clean, run, test
ns.add_collection(build)
ns.add_collection(clean)
ns.add_collection(run)
ns.add_collection(test)

ns.add_task(test.all, name='default', default=True)
