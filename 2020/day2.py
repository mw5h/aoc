#!/usr/bin/python

import sys

counter = 0

for entry in sys.stdin:
    policy, password = entry.rstrip().split(": ")
    rng, letter = policy.split()
    minimum, maximum = rng.split('-')
    idx1 = int(minimum) - 1
    idx2 = int(maximum) - 1
    if (password[idx1] == letter) != (password[idx2] == letter):
        counter = counter + 1

print counter
