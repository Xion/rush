"""
Run tasks.
"""
import os
import sys
import webbrowser

from invoke import task

from tasks import BIN
from tasks import build
from tasks._util import cargo, get_docs_output_dir


@task(default=True)
def bin():
    """Run the binary crate."""
    # Because we want to accept arbitrary arguments, we have to ferret them out
    # of sys.argv manually.
    cargo('run', *sys.argv[2:], crate=BIN, wait=False)


@task(pre=[build.docs])
def docs():
    """"Run" the docs, i.e. preview them in the default web browser."""
    path = os.path.join(get_docs_output_dir(), 'index.html')
    if sys.platform == 'darwin':
        path = 'file://%s' % os.path.abspath(path)
    webbrowser.open_new_tab(path)
