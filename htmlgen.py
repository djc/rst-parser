#!/usr/bin/env python
from __future__ import print_function
from docutils import core, writers
import os

def main():
    files = []
    writer = writers.get_writer_class('html5')()
    writer.default_stylesheets = []
    settings = {'template': 'tests/template.txt'}
    for fn in os.listdir('tests'):

        if not fn.endswith('.rst'):
            continue

        input_fn = os.path.join('tests', fn)
        with open(input_fn) as f:
            input = f.read()
        print('> processing %s...' % fn, end='')
        out = core.publish_string(input, writer=writer,
                                  settings_overrides=settings)
        out_fn = input_fn.replace('.rst', '.html')
        with open(out_fn, 'wb') as f:
            f.write(out)
        print(' done')

if __name__ == '__main__':
    main()
