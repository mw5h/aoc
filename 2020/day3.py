#!/usr/bin/python

import sys


data = [e.rstrip() for e in sys.stdin]

def f (right, down):
    counter = 0
    offset = 0
    skip = 0

    for i in range(0, len(data), down) :
        e = data[i]
        if offset >= len(e):
            offset = offset - len(e)
        if e[offset] == '#':
            counter = counter + 1
        offset = offset + right

    return counter

print f(1, 1) * f(3, 1) * f(5, 1) * f(7, 1) * f(1, 2)
