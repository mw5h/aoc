#!/usr/bin/python

import sys

data = [(op[0], int(op[1])) for op in [line.rstrip().split() for line in sys.stdin]]

isa = {'nop': lambda acc, ip, arg: (acc, ip + 1),
       'acc': lambda acc, ip, arg: (acc + arg, ip + 1),
       'jmp': lambda acc, ip, arg: (acc, ip + arg)}

def simulate(d):
    acc = 0
    ip = 0
    visited = set()
    while ip not in visited and ip < len(d):
        visited.add(ip)
        (acc, ip) = isa[d[ip][0]](acc, ip, d[ip][1])
    return (acc, visited, ip)

path = simulate(data)
print path[0]

flipper = {'nop' : 'jmp', 'jmp': 'nop', 'acc': 'acc'}
print [s[0] for s in [simulate([data[i] if i != flip else (flipper[data[i][0]], data[i][1]) for i in range(0, len(data))]) for flip in path[1]] if s[2] == len(data)][0]
