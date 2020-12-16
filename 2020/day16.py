#!/usr/bin/python

import sys
from operator import mul

sections = [sect.splitlines() for sect in ''.join(sys.stdin).split('\n\n')]

rules = {rule[0]: {valid for endpt in [term.split('-') for term in rule[1].split(' or ')]
                                                           for valid in range(int(endpt[0]), int(endpt[1]) + 1)}
                                                               for rule in [line.split(': ') for line in sections[0]]}
my_ticket = [int(v) for v in sections[1][1].split(',')]
nearby    = [[int(v) for v in ticket.split(',')] for ticket in sections[2][1:]]

valid_values = {v for rule in rules.values() for v in rule}
print 'part 1:', sum([res for near in nearby for res in (set(near) - valid_values)])

possible_fields = set(rules.keys())
valid_tickets = [ticket for ticket in nearby if set(ticket) <= valid_values]
valid_fields = [possible_fields - {k for k in set(rules.keys()) for ticket in valid_tickets if ticket[idx] not in rules[k]} for idx in range(0, len(my_ticket))]

resolved = {}
for (idx, _) in sorted(enumerate([len(field) for field in valid_fields]), None, lambda x: x[1]):
    field = valid_fields[idx] & possible_fields
    possible_fields -= field
    resolved[field.pop()] = idx

print 'part 2:', reduce(mul, [my_ticket[resolved[fieldname]] for fieldname in resolved.keys() if fieldname.startswith('departure')])
