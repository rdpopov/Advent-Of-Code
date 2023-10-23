#!/usr/bin/python3
from functools import reduce

# lens = [ord(x) for x in open('10.in','r').read().rstrip()]
lens = [83,0,193,1,254,237,187,40,88,27,2,255,149,29,42,100]
# print(lens)
# lens.extend([17,31,73,47,23])
nums = [x for x in range(0,256)]
pos = 0
skip = 0
# for _ in range(64):
for l in lens:
    to_reverse = []
    for x in range(l):
        n = (pos + x) % 256
        to_reverse.append(nums[n])
    to_reverse.reverse()
    for x in range(l):
        n = (pos + x) % 256
        nums[n] = to_reverse[x]
    pos += l + skip
    pos = pos % 256
    skip += 1
    print(nums)
# dense = []
# for x in range(0,16):
	# subslice = nums[16*x:16*x+16]
	# dense.append('%02x'%reduce((lambda x,y: x ^ y),subslice))

print(nums[0]*nums[1])

