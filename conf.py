# Minimal conf.py for Sphinx documentation

project = 'Finalytics'
author = 'Nnamdi Olisaeloka'
version = '0.1.8'
release = '0.1.8'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.doctest',
    'sphinx.ext.intersphinx',
    'sphinx.ext.viewcode',
]

templates_path = []
exclude_patterns = []

html_theme = 'alabaster'
html_static_path = []
