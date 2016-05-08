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

from invoke import call, task, run


BIN = 'rush'
LIB = 'librush'


@task(name="run")
def run_():
    """Run the binary crate."""
    # Because we want to accept arbitrary arguments, we have to ferret them out
    # of sys.argv manually.
    cargo('run', *sys.argv[2:], crate=BIN, wait=False)


@task(auto_shortflags=False,
      help={'what': "Comma-separated list of targets to build: bin,docs,lib",
            'release': "Whether to build artifacts in release mode"})
def build(what, release=False):
    """Build the project."""
    targets = resolve(what, all=('bin', 'docs', 'lib'))

    if 'docs' in targets:
        args = ['--clean'] if release else []
        run('mkdocs build ' + ' '.join(map(quote, args)), pty=True)
    targets.discard('docs')

    for crate in map(crate_from_target, targets):
        args = ['--release'] if release else []
        cargo('build', *args, crate=crate, pty=True)


@task(help={'what': "Comma-separated list of targets to test: bin,lib"})
def test(what):
    """Run the project's tests."""
    targets = resolve(what, all=('bin', 'lib'))
    for crate in map(crate_from_target, targets):
        cargo('test', '--no-fail-fast', crate=crate, pty=True)


@task
def release():
    """Create the release packages for various operating systems."""
    run('./tools/release', pty=True)


@task(default=True, autoprint=True,
      pre=[call(test, 'all'), call(build, 'docs')])
def default():
    """Default task: run all the tests & build documentation."""
    return "\nAll done."


# Utility functions

# TODO(xion): consider using the Invoke's collection feature instead of this,
# so that we docs can be handled wit something like `inv build.docs`
# (details: http://docs.pyinvoke.org/en/0.12.2/concepts/namespaces.html)
def resolve(what, all=None):
    """Resolve the target specification given as task argument
    into a set of target names.

    :param what: Comma-separated string, like "bin,lib"
    :param all: What targets are considered 'all' (CSV or list)

    :return: Set of resolved targets
    """
    all_ = all or ('bin', 'lib')

    result = what.lower().strip()
    if result == 'all':
        result = all_

    if isinstance(result, str):
        result = result.split(',')
    return set(t.lower().strip() for t in result)


def crate_from_target(target):
    """Translate a target name (like 'bin') into the appropriate crate name."""
    try:
        return {'bin': BIN, 'lib': LIB}[target]
    except KeyError:
        raise ValueError("unknown target: '%s'" % (target,))


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
