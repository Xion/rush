"""
Miscellaneous tasks.
"""
from __future__ import print_function

from glob import glob

from invoke import task

from tasks import LIB


@task(name="print-grammar")
def print_grammar(ctx):
    """Prints the language's grammar rules.
    Uses comments in the syntax definition & parsing code.
    """
    syntax_package = './crates/%s/src/parse/syntax' % (LIB,)
    for filename in glob('%s/*.rs' % syntax_package):
        with open(filename) as f:
            for line in f.readlines():
                # TODO: support grammar rules spanning more than one line
                if '::==' in line:
                    line = line.lstrip('/').strip()
                    print(line)
