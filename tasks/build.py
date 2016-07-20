"""
Build tasks.
"""
from __future__ import print_function

try:
    from shlex import quote
except ImportError:
    from pipes import quote

from invoke import task

from tasks import BIN, LIB
from tasks._util import cargo


HELP = {'release': "Whether to build artifacts in release mode"}


@task(help=HELP, default=True)
def all(ctx, release=False):
    """Build the project."""
    # calling build_lib() is unnecessary because the binary crate
    # depeends on the library, so it will be rebuilt as well
    bin(release)
    docs(release)
    print("\nBuild finished.")


@task(help=HELP)
def bin(ctx, release=False):
    """Build the binary crate."""
    args = ['--release'] if release else []
    cargo(ctx, 'build', *args, crate=BIN, pty=True)


@task(help=HELP)
def lib(ctx, release=False):
    """Build the library crate."""
    args = ['--release'] if release else []
    cargo(ctx, 'build', *args, crate=LIB, pty=True)


@task(help=HELP)
def docs(ctx, release=False):
    """Build the project documentation."""
    args = ['--clean'] if release else []
    ctx.run('mkdocs build ' + ' '.join(map(quote, args)), pty=True)
