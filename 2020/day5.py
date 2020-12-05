#!/usr/bin/python

import sys


data = [e.rstrip() for e in sys.stdin]
trans = {'F': 0, 'B': 1, 'R': 1, 'L': 0}

def seatNumber(spec):
    seat = 0
    for c in spec:
        seat = seat * 2 + trans[c]
    return seat

seats = [seatNumber(spec) for spec in data]

print max(seats)

seatset = frozenset(seats)

print [x + 1 for x in seats if x + 2 in seatset and x + 1 not in seatset][0]
