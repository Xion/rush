"""
Lint tasks.
"""
from invoke import task


@task
def tasks(ctx):
    """Lint the Invoke tasks' code."""
    return ctx.run('flake8 tasks').return_code


@task(default=True, pre=[tasks])
def all(ctx):
    """Run all the lint tasks."""
