"""
Helper code related to generating documentation for the project.
"""
from collections import namedtuple, OrderedDict
import io
import logging
import os
import re
import sys

import yaml
from glob2 import glob


__all__ = ['get_docs_output_dir']


def get_docs_output_dir():
    """Retrieve the full path to the documentation's output directory."""
    base_dir = os.getcwd()
    config_file = os.path.join(base_dir, 'mkdocs.yml')
    if not os.path.exists(config_file):
        logging.error("mkdocs.yaml config file cannot be found; "
                      "is it the project's root directory?")
        sys.exit(1)
    with io.open(config_file, encoding='utf-8') as f:
        config = yaml.load(f)
    return os.path.join(base_dir, config.get('site_dir', 'site'))


# Generating API docs

#: Module under eval::api that contains API functions.
Module = namedtuple('Module', [
    'path',  # full path to the module file
    'name',
    'submodules',  # list of Module objects
    'functions',  # list of Function objects
])

#: API function that should be described in the end-user documentation.
Function = namedtuple('Function', [
    'name',
    'description',  # general description of the function
    'arguments',  # OrderedDict of names->descriptions
    'returns',  # description of the return value
])


def describe_rust_api(*src):
    """Describe the public API implemented in given Rust modules.

    :param src: Rust source file(s) to analyze.
                This can be a list of file paths or a glob string.

    :return: Iterable of Module objects
    """
    # collapse the possible lists of files into a single list
    sources = [[s] if isinstance(s, str) else s for s in src]
    sources = sum(sources, [])
    if not sources:
        return

    for source in sources:
        for filename in glob(source):
            module = analyze_rust_module(filename)
            if module:
                yield module


def analyze_rust_module(path):
    """Analyze given Rust module file.
    :param path: Path to the module
    :return: Module object or None
    """
    logging.info("Analyzing Rust module %s...", path)
    with io.open(path, encoding='utf-8') as f:
        lines = f.readlines()

    functions = []

    pub_fn_line_indices = (i for i, line in enumerate(lines)
                           if line.lstrip().startswith('pub fn'))
    for idx in pub_fn_line_indices:
        def_line = lines[idx]

        # extract function name
        fn_name_match = re.match(r'pub\s+fn\s+(\w+)\(', def_line)
        if not fn_name_match:
            logging.warning(
                "Spurious Rust function definition line: %s", def_line)
            continue
        fn_name = fn_name_match.group(1)

        # TODO: extract argument names

        # extract documentation
        docstring_lines = []
        for j in range(idx - 1, 0, -1):
            line = lines[j].lstrip()
            if not line.startswith('///'):
                break
            # treat empty lines as paragraph separators
            line = line.lstrip('/').strip() or os.linesep
            docstring_lines.append(line)
        docstring = ''.join(docstring_lines)

        # TODO: support some kind of docstring tags that'd describe
        # arguments and the return value
        func = Function(name=fn_name,
                        description=docstring,
                        arguments=OrderedDict(),
                        returns=None)

        logging.debug(
            "Found function %s(%s) -> %s",
            func.name, ', '.join(func.arguments), func.returns or "?")
        functions.append(func)

    mod_name, _ = os.path.splitext(os.path.basename(path))
    module = Module(path=path,
                    name=mod_name,  # TODO: parent directory name if mod.rs
                    submodules=[],  # TODO: descend to submodules if mod.rs
                    functions=functions)

    logging.info("Module %s had %s function(s) and %s submodule(s)",
                 module.name, len(module.functions), len(module.functions))
    return module
