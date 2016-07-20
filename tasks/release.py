"""
Release tasks.
"""
from invoke import task


# TODO: rewrite the ./tools/release script in Python into nested tasks
# like release.deb or release.rpm or release.brew
@task(default=True)
def all(ctx):
    """Create the release packages for various operating systems."""
    ctx.run('./tools/release', pty=True)
