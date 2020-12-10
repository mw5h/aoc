#!/usr/bin/python

import sys
from collections import Counter

data = sorted([0] + [int(d) for d in sys.stdin])
intervals = Counter([data[i] - data[i - 1] for i in range(1, len(data))])
print intervals[1] * (intervals[3] + 1)

p = {max(data) + 3 : 1}
for a in reversed(data):
    p[a] = sum([p[i] for i in range(a+1, a+4) if i in p])
print p[0]
