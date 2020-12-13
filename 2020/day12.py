#!/usr/bin/python

import sys
from math import sin, cos, radians
from functools import reduce
from collections import namedtuple

data = [(l[0], int(l[1:])) for l in sys.stdin]
Location = namedtuple('Location', ['x', 'y', 'd', 'sx', 'sy'])

rules = {'N': lambda s, v: s._replace(y = s.y + v),
         'S': lambda s, v: s._replace(y = s.y - v),
         'E': lambda s, v: s._replace(x = s.x + v),
         'W': lambda s, v: s._replace(x = s.x - v),
         'R': lambda s, v: s._replace(d = s.d + v),
         'L': lambda s, v: s._replace(d = s.d - v),
         'F': lambda s, v: s._replace(x = s.x + int(sin(radians(s.d))) * v, y = s.y + int(cos(radians(s.d))) * v)}

start = Location(x=0, y=0, d=90, sx=0, sy=0)
pos = reduce(lambda s, i: rules[i[0]](s, i[1]), data, start)
print 'part 1: ', abs(pos.x) + abs(pos.y)

rot = [lambda s: s,
       lambda s: s._replace(x = s.y, y = -s.x),
       lambda s: s._replace(x = -s.x, y = -s.y),
       lambda s: s._replace(x = -s.y, y = s.x)]
rules['R'] = lambda s, v: rot[v/90](s)
rules['L'] = lambda s, v: rot[(360-v)/90](s)
rules['F'] = lambda s, v: s._replace(sx = s.sx + s.x * v, sy = s.sy + s.y *v)

start = Location(x=10, y=1, d=0, sx=0, sy=0)
pos = reduce(lambda s, i: rules[i[0]](s, i[1]), data, start)
print 'part 2: ', abs(pos.sx) + abs(pos.sy)
