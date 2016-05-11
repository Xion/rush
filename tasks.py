"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
from __future__ import absolute_import

import logging
import os
try:
    from shlex import quote
except ImportError:
    from pipes import quote
import shutil
import sys

from invoke import Collection, task, run
import yaml


BIN = 'rush'
LIB = 'librush'


@task(name="run")
def run_():
    """Run the binary crate."""
    # Because we want to accept arbitrary arguments, we have to ferret them out
    # of sys.argv manually.
    cargo('run', *sys.argv[2:], crate=BIN, wait=False)


@task
def release():
    """Create the release packages for various operating systems."""
    run('./tools/release', pty=True)


# Build tasks

BUILD_HELP = {'release': "Whether to build artifacts in release mode"}


@task(help=BUILD_HELP)
def build_all(release=False):
    """Build the project."""
    # calling build_lib() is unnecessary because the binary crate
    # depeends on the library, so it will be rebuilt as well
    build_bin(release)
    build_docs(release)
    print("\nBuild finished.")


@task(help=BUILD_HELP)
def build_bin(release=False):
    """Build the binary crate."""
    args = ['--release'] if release else []
    cargo('build', *args, crate=BIN, pty=True)


@task(help=BUILD_HELP)
def build_lib(release=False):
    """Build the library crate."""
    args = ['--release'] if release else []
    cargo('build', *args, crate=LIB, pty=True)


@task(help=BUILD_HELP)
def build_docs(release=False):
    """Build the project documentation."""
    args = ['--clean'] if release else []
    run('mkdocs build ' + ' '.join(map(quote, args)), pty=True)


# Test tasks

@task
def test_all():
    """Execute the project's tests."""
    test_lib()
    test_bin()


@task
def test_bin():
    """Execute the binary crate's tests."""
    cargo('test', '--no-fail-fast', crate=BIN, pty=True)


@task
def test_lib():
    """Execute the library crate's tests."""
    cargo('test', '--no-fail-fast', crate=LIB, pty=True)


# Clean tasks

CLEAN_HELP = {'release': "Whether the to clean release artifacts."}


@task(help=CLEAN_HELP)
def clean_all(release=False):
    """Clean all of the project's build artifacts."""
    clean_lib(release)
    clean_bin(release)
    clean_docs()
    print("\nAll cleaned.")


@task(help=CLEAN_HELP)
def clean_bin(release=False):
    """Clean the binary crate's build artifacts."""
    args = ['--release'] if release else []
    cargo('clean', *args, crate=BIN, pty=True)


@task
def clean_docs():
    """Clean the built documentation."""
    # determine the docs' output directory
    base_dir = os.path.dirname(__file__)
    config_file = os.path.join(base_dir, 'mkdocs.yml')
    with open(config_file) as f:
        config = yaml.load(f)
    output_dir = os.path.join(base_dir, config.get('site_dir', 'site'))

    if os.path.isdir(output_dir):
        try:
            shutil.rmtree(output_dir)
        except (OSError, shutil.Error) as e:
            logging.warning("Error while cleaning docs' output dir: %s", e)


@task(help=CLEAN_HELP)
def clean_lib(release=False):
    """Clean the library crate's build artifacts."""
    args = ['--release'] if release else []
    cargo('clean', *args, crate=LIB, pty=True)


# Utility functions

def cargo(cmd, *args, **kwargs):
    """Run Cargo as if inside the specified crate directory.

    :param crate: Name of the crate to run Cargo against
    :param wait: Whether to wait for the Cargo process to finish (True),
                 or to replace the whole Invoke process with it (False)
    """
    cargo_args = [cmd]

    crate = kwargs.pop('crate', None)
    if crate:
        cargo_args.append('--manifest-path')
        cargo_args.append(os.path.join('crates', crate, 'Cargo.toml'))

    cargo_args.extend(args)

    wait = kwargs.pop('wait', True)
    if wait:
        return run('cargo ' + ' '.join(map(quote, cargo_args)), **kwargs)
    else:
        argv = ['cargo'] + cargo_args  # execvp() needs explicit argv[0]
        os.execvp('cargo', argv)


# Setup

ns = Collection(run_, release)
ns.add_task(test_all, name='default', default=True)

build_tasks = Collection('build',
                         bin=build_bin, docs=build_docs, lib=build_lib)
build_tasks.add_task(build_all, name='all', default=True)
ns.add_collection(build_tasks)

test_tasks = Collection('test',
                        bin=test_bin, lib=test_lib)
test_tasks.add_task(test_all, name='all', default=True)
ns.add_collection(test_tasks)

clean_tasks = Collection('clean',
                         bin=clean_bin, docs=clean_docs, lib=clean_lib)
clean_tasks.add_task(clean_all, name='all', default=True)
ns.add_collection(clean_tasks)
