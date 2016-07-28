"""
Lint tasks.
"""
from invoke import task


@task(default=True)
def all(ctx):
    """Run all the lint tasks."""
    tasks(ctx)


@task
def tasks(ctx):
    """Lint the Invoke tasks' code."""
    ctx.run('flake8 tasks')
