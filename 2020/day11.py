#!/usr/bin/python

import sys

data = [[s for s in l.rstrip()] for l in sys.stdin]

adjacencies = [[[(j, i) for i in range(x - 1, x + 2)
                            for j in range(y - 1, y + 2)
                                if (j != y or i != x) and 0 <= j < len(data) and 0 <= i < len(data[j])]
                                    for x in range(0, len(data[y]))]
                                        for y in range(0, len(data))]

def sim(layout, f):
    while True:
        dst = [[f(layout, y, x) for x in range(0, len(layout[y]))] for y in range(0, len(layout))]
        if dst == layout:
            return sum([r.count('#') for r in dst])
        layout = dst

def step_seat(adjmap, layout, rules, y, x):
    return rules[layout[y][x]]([layout[j][i] for (j, i) in adjmap[y][x]].count('#'))

rules = {'L' : lambda c : '#' if c == 0 else 'L',
         '#' : lambda c : 'L' if c >= 4 else '#',
         '.' : lambda c : '.'}

print 'part 1: ', sim(data, lambda l, y, x: step_seat(adjacencies, l, rules, y, x))

def first_seat(layout, y, x, dy, dx):
    while True:
        y += dy
        x += dx
        if not 0 <= y < len(layout) or not 0 <= x < len(layout[y]):
            return None
        if layout[y][x] != '.':
            return (y, x)

los = [[[fs for fs in [first_seat(data, y, x, j, i) for j in range(-1, 2)
                                                        for i in range(-1, 2) if j != 0 or i != 0]
            if fs is not None]
                for x in range(0, len(data[y]))]
                    for y in range(0, len(data))]
rules['#'] = lambda c : 'L' if c >= 5 else '#'

print 'part 2: ', sim(data, lambda l, y, x: step_seat(los, l, rules, y, x))
