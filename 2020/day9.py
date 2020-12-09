#!/usr/bin/python

import sys

data = [int(d) for d in sys.stdin]

invalid = [data[i] for i in range(25, len(data)) if len([j for j in data[i - 25 : i] if data[i] - j in set(data[i - 25 : i])]) == 0][0]
print invalid
print [max(data[i : j]) + min(data[i : j]) for i in range(0, len(data)) for j in range(i + 2, len(data)) if invalid == sum(data[i : j])][0]
