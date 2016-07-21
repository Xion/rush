"""
Helper code related to generating documentation for the project.
"""
import logging
import os
import sys

import yaml


__all__ = ['get_docs_output_dir']


def get_docs_output_dir():
    """Retrieve the full path to the documentation's output directory."""
    base_dir = os.getcwd()
    config_file = os.path.join(base_dir, 'mkdocs.yml')
    if not os.path.exists(config_file):
        logging.error("mkdocs.yaml config file cannot be found; "
                      "is it the project's root directory?")
        sys.exit(1)
    with open(config_file) as f:
        config = yaml.load(f)
    return os.path.join(base_dir, config.get('site_dir', 'site'))
