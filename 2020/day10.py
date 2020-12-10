#!/usr/bin/python

import sys
from collections import Counter, defaultdict

data = sorted([0] + [int(d) for d in sys.stdin])
intervals = Counter([data[i] - data[i - 1] for i in range(1, len(data))])
print intervals[1] * (intervals[3] + 1)

p = defaultdict(lambda: 0, [(max(data) + 3, 1)])
for a in reversed(data):
    p[a] = p[a+1] + p[a+2] + p[a+3]
print p[0]
