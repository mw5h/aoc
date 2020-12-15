#!/usr/bin/python

from functools import reduce
data = [19,0,5,1,10,13]

def onestep(s, t):
    v = data[t] if t < len(data) else t - s[1][s[0]] - 1 if s[0] in s[1] else 0
    s[1][s[0]] = t - 1
    return (v, s[1])

print 'part 1:', reduce(onestep, range(0, 2020), (None, {}))[0]
print 'part 2:', reduce(onestep, range(0, 30000000), (None, {}))[0]
