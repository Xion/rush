"""
Test tasks.
"""
from invoke import task

from tasks import BIN, LIB
from tasks.util import cargo


@task(default=True)
def all(ctx):
    """Execute the project's tests."""
    lib(ctx)
    bin(ctx)


@task
def bin(ctx):
    """Execute the binary crate's tests."""
    return cargo(
        ctx, 'test', '--no-fail-fast', crate=BIN, pty=True).return_code


@task
def lib(ctx):
    """Execute the library crate's tests."""
    return cargo(
        ctx, 'test', '--no-fail-fast', crate=LIB, pty=True).return_code
