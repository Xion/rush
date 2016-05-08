"""
Automation tasks, aided by the Invoke package.

Most of them mimic the Rust's Cargo binary, but they do what makes sense for
either the library crate, binary crate, or both.
"""
import os
try:
    from shlex import quote
except ImportError:
    from pipes import quote
import sys

from invoke import task, run


BIN = 'rush'
LIB = 'librush'


@task(name="run")
def run_():
    """Run the binary crate."""
    # Because we want to accept arbitrary arguments, we have to ferret them out
    # of sys.argv manually.
    cargo('run', *sys.argv[2:], crate=BIN, wait=False)


@task
def build(release=False):
    """Build the binary crate."""
    args = ['--release'] if release else []
    cargo('build', *args, crate=BIN, pty=True)


@task(default=True)
def test():
    """Run the tests for both binary & library crates."""
    for crate in (LIB, BIN):
        cargo('test', crate=crate, pty=True)


@task
def release():
    """Create the release packages for various operating systems."""
    run('./tools/release', pty=True)


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
