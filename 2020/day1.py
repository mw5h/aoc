#!/usr/bin/python

import sys

data = [int(v) for v in sys.stdin]
s = set(data)
data.sort()

for i in range(0, len(data)):
    v1 = data[i]
    for j in range(i, len(data)):
        v2 = data[j]
        diff = 2020 - (v1 + v2)
        if diff in s:
            print v1 * v2 * diff
            break
        elif diff <= 0:
            break
