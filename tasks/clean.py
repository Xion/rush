"""
Clean tasks.
"""
from __future__ import print_function

import logging
import os
import shutil
import sys

from invoke import task

from tasks import BIN, LIB
from tasks.util import cargo
from tasks.util.docs import get_docs_output_dir


HELP = {'release': "Whether the to clean release artifacts."}


@task(help=HELP, default=True)
def all(ctx, release=False):
    """Clean all of the project's build artifacts."""
    lib(ctx, release=release)
    bin(ctx, release=release)
    docs(ctx)
    print("\nAll cleaned.", file=sys.stderr)


@task(help=HELP)
def bin(ctx, release=False):
    """Clean the binary crate's build artifacts."""
    args = ['--release'] if release else []
    cargo(ctx, 'clean', *args, crate=BIN, pty=True)


@task
def docs(ctx):
    """Clean the built documentation."""
    output_dir = get_docs_output_dir()
    if os.path.isdir(output_dir):
        try:
            shutil.rmtree(output_dir)
        except (OSError, shutil.Error) as e:
            logging.warning("Error while cleaning docs' output dir: %s", e)


@task(help=HELP)
def lib(ctx, release=False):
    """Clean the library crate's build artifacts."""
    args = ['--release'] if release else []
    cargo(ctx, 'clean', *args, crate=LIB, pty=True)
