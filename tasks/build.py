"""
Build tasks.
"""
from __future__ import print_function

import logging
try:
    from shlex import quote
except ImportError:
    from pipes import quote
import sys

from invoke import task
import semver

from tasks import BIN, LIB
from tasks.util import cargo


MIN_RUSTC_VERSION = '1.10.0'

HELP = {'release': "Whether to build artifacts in release mode"}


@task(help=HELP, default=True)
def all(ctx, release=False):
    """Build the project."""
    # calling build_lib() is unnecessary because the binary crate
    # depeends on the library, so it will be rebuilt as well
    bin(ctx, release)
    docs(ctx, release)
    print("\nBuild finished.")


@task(help=HELP)
def bin(ctx, release=False):
    """Build the binary crate."""
    ensure_rustc_version(ctx)
    args = ['--release'] if release else []
    cargo(ctx, 'build', *args, crate=BIN, pty=True)


@task(help=HELP)
def lib(ctx, release=False):
    """Build the library crate."""
    ensure_rustc_version(ctx)
    args = ['--release'] if release else []
    cargo(ctx, 'build', *args, crate=LIB, pty=True)


@task(help=HELP)
def docs(ctx, release=False):
    """Build the project documentation."""
    args = ['--clean'] if release else []
    ctx.run('mkdocs build ' + ' '.join(map(quote, args)), pty=True)


# Utility functions

def ensure_rustc_version(ctx):
    """Terminates the build unless the Rust compiler is recent enough."""
    rustc_v = ctx.run('rustc --version', hide=True)
    if not rustc_v.ok:
        logging.critical("Rust compiler not found, aborting build.")
        sys.exit(127)

    _, version, _ = rustc_v.stdout.split(None, 2)
    if not semver.match(version, '>=' + MIN_RUSTC_VERSION):
        logging.error("Build requires at least Rust %s, found %s",
                      MIN_RUSTC_VERSION, version)
        sys.exit(1)

    return True
