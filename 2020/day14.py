#!/usr/bin/python

import sys
import re
from string import maketrans
from functools import partial, reduce

memdecode = re.compile(r'^mem\[(\d+)\]')
decoder = {'mem' : lambda k, v: ('mem', int(re.match(memdecode, k).group(1)), int(v)),
           'mas' : lambda k, v: ('mask', v)}
data = [decoder[ins[0:3]](ins, val) for (ins, val) in [l.rstrip().split(' = ') for l in sys.stdin]]

def update_mem(mem, loc, val):
    for l in loc:
        mem[l] = val
    return mem

def makemask(trans, mask, mask2=None):
    mask2 = mask if mask2 is None else mask2
    return partial(lambda m, v: (v & m[1]) | m[0], (int(mask.translate(maketrans('X10', trans[0])), 2),
                                                    int(mask2.translate(maketrans('X10', trans[1])), 2)))

isa = {'mem':  lambda s, v: (s[0], update_mem(s[1], [v[0]], s[0](v[1]))),
       'mask': lambda s, v: (makemask(('010', '100'), v[0]), s[1])}
print 'part 1:', sum(reduce(lambda s, v: isa[v[0]](s, v[1:]), data, (None, {}))[1].values())

def permutemask(mask, orig):
    if 'X' not in mask:
        return [makemask(('110', '001'), mask, orig)]
    return permutemask(mask.replace('X', '0', 1), orig) + permutemask(mask.replace('X', '1', 1), orig)

isa = {'mem'  : lambda s, v: (s[0], update_mem(s[1], [m(v[0]) for m in s[0]], v[1])),
       'mask' : lambda s, v: (permutemask(v[0], v[0]), s[1])}
print 'part 2:', sum(reduce(lambda s, v: isa[v[0]](s, v[1:]), data, (None, {}))[1].values())
