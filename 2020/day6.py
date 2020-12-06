#!/usr/bin/python

import sys

data = [[set([q for q in e]) for e in g.split('\n') if len(e) > 0] for g in ''.join(sys.stdin).split('\n\n')]
print sum([len(set.union(*g)) for g in data])
print sum([len(set.intersection(*g)) for g in data])
