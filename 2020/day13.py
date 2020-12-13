#!/usr/bin/python

import sys
from functools import reduce
from itertools import count
from operator import mul

data = [l for l in sys.stdin]
print 'part 1:', mul(*min([(bus - int(data[0]) % bus, bus) for bus in [int(b) for b in data[1].split(',') if b != 'x']]))

def onebus(state, bus):
    if state is None:
        return (-bus[0], bus[1])
    for t in count(state[0], state[1]):
        if (t + bus[0]) % bus[1] == 0:
            return (t, state[1] * bus[1])

print 'part 2:', reduce(onebus, reversed(sorted([(b[0], int(b[1])) for b in enumerate(data[1].split(',')) if b[1] != 'x'], key=lambda t: t[1])), None)[0]
