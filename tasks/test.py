"""
Test tasks.
"""
from invoke import task

from tasks import BIN, LIB
from tasks._util import cargo


@task(default=True)
def all():
    """Execute the project's tests."""
    lib()
    bin()


@task
def bin():
    """Execute the binary crate's tests."""
    cargo('test', '--no-fail-fast', crate=BIN, pty=True)


@task
def lib():
    """Execute the library crate's tests."""
    cargo('test', '--no-fail-fast', crate=LIB, pty=True)
