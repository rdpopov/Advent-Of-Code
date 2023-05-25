#!/usr/bin/python3
import itertools
import functools
import operator

def gen_sum(pool, current, sum):
    if sum == 0:
        yield current
    elif sum > 0 and len(pool) > 0:
        yield from gen_sum(pool[1:], current + [pool[0]], sum - pool[0])
        yield from gen_sum(pool[1:], current, sum)

def mult(l):
    return functools.reduce(operator.mul, l, 1)

def solve(file, n_sums):
    with open(file) as f:
        nums = list(reversed([int(x.strip()) for x in f.readlines()]))
        sum_limit = sum(nums,0) // n_sums
        min_qe = (-1, len(nums))
        for i in gen_sum(nums,[] , sum_limit):
            if min_qe[0] < 0:
                min_qe = (mult(i),len(i))
            curr = (mult(i),len(i))
            if curr[0] < min_qe[0] and curr[1] <= min_qe[1]:
                min_qe = curr
        return min_qe


if __name__ == "__main__":
    print("tests")
    print("part1 ", solve("./input1",3))
    print("part2 ", solve("./input1",4))
    print("real input")
    print("part1 ", solve("./input",3))
    print("part2 ", solve("./input",4))

