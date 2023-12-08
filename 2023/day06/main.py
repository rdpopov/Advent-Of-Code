from functools import reduce
from operator import mul

from math import floor, ceil


def parse_inut(fname):
    inp = {}
    with open(fname) as f:
        for i in f.readlines():
            k, v = i.split(':')
            inp[k] = list(map(int, v.split()))
    return inp


def parse_inut2(fname):
    inp = {}
    with open(fname) as f:
        for i in f.readlines():
            k, v = i.split(':')
            inp[k] = [int("".join(v.split()))]
    return inp


def between(mT, r):
    sq = (mT*mT - 4*r) ** 0.5
    return [floor((mT - sq)/2 + 1), ceil((mT + sq)/2 - 1)]


def part1(fname: str, parse_func) -> int:
    inp = parse_func(fname)
    seq = map(lambda x: between(*x), zip(inp["Time"], inp["Distance"]))
    return reduce(mul, map(lambda x: x[1]-x[0]+1, seq), 1)


def main():
    print("Part 1: ", part1("test", parse_inut))
    print("Part 1: ", part1("input", parse_inut))
    print("Part 2: ", part1("test", parse_inut2))
    print("Part 2: ", part1("input", parse_inut2))


main()
