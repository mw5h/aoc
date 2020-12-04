#!/usr/bin/python

import sys
import re

hcl = re.compile('^#[0-9a-f]{6}$')
ecl = frozenset(['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'])
pid = re.compile('^\d{9}$')

v = {'byr': lambda x: 1920 <= int(x) <= 2002,
     'iyr': lambda x: 2010 <= int(x) <= 2020,
     'eyr': lambda x: 2020 <= int(x) <= 2030,
     'hgt': lambda x: (x.endswith('cm') and 150 <= int(x[:-2]) <= 193) or (x.endswith('in') and 59 <= int(x[:-2]) <= 76),
     'hcl': lambda x: hcl.match(x) is not None,
     'ecl': lambda x: x in ecl,
     'pid': lambda x: pid.match(x) is not None,
     'cid': lambda x: True}

data = [dict([k.split(':') for k in e.split()]) for e in  ''.join(sys.stdin).split('\n\n')]
for p in data:
    p.setdefault('cid', None)
req  = set(['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid'])
valid  = [e for e in data if len(set(e.keys()) ^ req) == 0]

print len(valid)

valid = [e for e in data if len([k for k in v.keys() if k in e and v[k](e[k])]) == len(v.keys())]

print len(valid)
