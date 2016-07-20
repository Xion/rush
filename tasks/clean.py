"""
Clean tasks.
"""
import logging
import os
import shutil

from invoke import task

from tasks import BIN, LIB
from tasks._util import cargo, get_docs_output_dir


HELP = {'release': "Whether the to clean release artifacts."}


@task(help=HELP, default=True)
def all(release=False):
    """Clean all of the project's build artifacts."""
    lib(release)
    bin(release)
    docs()
    print("\nAll cleaned.")


@task(help=HELP)
def bin(release=False):
    """Clean the binary crate's build artifacts."""
    args = ['--release'] if release else []
    cargo('clean', *args, crate=BIN, pty=True)


@task
def docs():
    """Clean the built documentation."""
    output_dir = get_docs_output_dir()
    if os.path.isdir(output_dir):
        try:
            shutil.rmtree(output_dir)
        except (OSError, shutil.Error) as e:
            logging.warning("Error while cleaning docs' output dir: %s", e)


@task(help=HELP)
def lib(release=False):
    """Clean the library crate's build artifacts."""
    args = ['--release'] if release else []
    cargo('clean', *args, crate=LIB, pty=True)
