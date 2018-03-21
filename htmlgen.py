#!/usr/bin/env python
from __future__ import print_function
from docutils import core
import os

SETTINGS = {
    'template': 'tests/template.txt',
}

def main():
    for fn in os.listdir('tests'):

        if not fn.endswith('.rst'):
            continue

        input_fn = os.path.join('tests', fn)
        with open(input_fn) as f:
            input = f.read()
        print('> processing %s...' % fn, end='')
        out = core.publish_string(input, writer_name='html5',
                                  settings_overrides=SETTINGS)
        out = out.rstrip() + '\n'
        out_fn = input_fn.replace('.rst', '.html')
        with open(out_fn, 'wb') as f:
            f.write(out)
        print(' done')

if __name__ == '__main__':
    main()
