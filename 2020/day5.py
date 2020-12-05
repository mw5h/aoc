#!/usr/bin/python

import sys
import string

trans = string.maketrans("fbrlFBRL", "01100110")
data = frozenset([int(e.rstrip().translate(trans), 2) for e in sys.stdin])

print max(data)

print [x + 1 for x in data if x + 2 in data and x + 1 not in data][0]
