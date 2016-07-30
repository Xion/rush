"""
Utility functions used by multiple task collections.
"""
import os
try:
    from shlex import quote
except ImportError:
    from pipes import quote


__all__ = ['cargo']


def cargo(ctx, cmd, *args, **kwargs):
    """Run Cargo as if inside the specified crate directory.

    :param ctx: Invoke's Context object
    :param cmd: Cargo command to run

    The following are optional keyword arguments:

    :param crate: Name of the crate to run Cargo against
    :param wait: Whether to wait for the Cargo process to finish (True),
                 or to replace the whole Invoke process with it (False)

    :return: If wait=True, the Invoke's Result object of the run.
    """
    cargo_args = [cmd]

    crate = kwargs.pop('crate', None)
    if crate:
        cargo_args.append('--manifest-path')
        cargo_args.append(os.path.join('crates', crate, 'Cargo.toml'))

    cargo_args.extend(args)

    wait = kwargs.pop('wait', True)
    if wait:
        return ctx.run('cargo ' + ' '.join(map(quote, cargo_args)), **kwargs)
    else:
        argv = ['cargo'] + cargo_args  # execvp() needs explicit argv[0]
        os.execvp('cargo', argv)
