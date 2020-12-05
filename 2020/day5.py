#!/usr/bin/python

import sys
import string

trans = string.maketrans("fbrlFBRL", "01100110")
data = [int(e.rstrip().translate(trans), 2) for e in sys.stdin]

print max(data)

seatset = frozenset(data)

print [x + 1 for x in data if x + 2 in seatset and x + 1 not in seatset][0]
