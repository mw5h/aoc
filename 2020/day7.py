#!/usr/bin/python

import sys
import re

data = {rule[0]: {i.group(2): int(i.group(1)) for i in re.finditer(r'(\d+) (.+?) bag', rule[1])} for rule in  [l.rstrip().split(' bags contain ') for l in sys.stdin]}

def part1(c):
   visit = set([c])
   visited = set()

   while len(visit):
       visited |= visit
       visit = {r[0] for r in data.items() if len(visit & set(r[1].keys()))}
   return len(visited)

def part2(c):
    return sum([k[1] * part2(k[0]) for k in data[c].items()]) + 1

print part1('shiny gold') - 1
print part2('shiny gold') - 1
