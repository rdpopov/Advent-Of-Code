from functools import partial
from functools import reduce
from itertools import cycle
import operator
import re


def parse_inut(fname):
    inp = {}
    with open(fname) as f:
        for i, v in enumerate(f.readlines()):
            if i == 0:
                fst = v.strip()
                continue
            if i == 1:
                continue
            k = list(filter(lambda x: len(x) > 0, re.split(r"[,() =\n]", v)))
            inp[k[0]] = k[1:]
    return fst, inp


def iterations_for_start(instr, tbl, is_end, start):
    crnt = start
    itr = 0
    way = cycle(instr)
    while is_end(crnt):
        turn = way.__next__()
        if turn == 'L':
            crnt = tbl[crnt][0]
        else:
            crnt = tbl[crnt][1]
        itr += 1
    return itr


def part1(fname: str, parse_func) -> int:
    instr, tbl = parse_func(fname)
    return iterations_for_start(instr, tbl, lambda x: x != "ZZZ", "AAA")


def prime_factors(n):
    dv = 2
    res = []
    N = n
    while N > 1 or dv <= N:
        if N % dv == 0:
            res.append(dv)
            N = N / dv
        else:
            dv += 1
    return res


def part2(fname: str, parse_func) -> int:
    instr, tbl = parse_func(fname)
    length_to_end = partial(iterations_for_start, instr,
                            tbl, lambda x: x[-1] != "Z")
    path_len = filter(lambda x: x[-1] == 'A', tbl.keys())
    path_len = list(path_len)
    path_len = set(map(length_to_end, path_len))
    factors = set()
    for i in path_len:
        for j in prime_factors(i):
            factors.add(j)

    return reduce(operator.mul, factors, 1)


def main():
    print("Part 1: ", part1("test", parse_inut))
    print("Part 1: ", part1("test2", parse_inut))
    print("Part 2: ", part2("test3", parse_inut))
    print("Part 1: ", part1("input", parse_inut))
    print("Part 2: ", part2("input", parse_inut))


main()
