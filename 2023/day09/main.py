import numpy as np


def parse_inut(fname):
    inp = []
    with open(fname) as f:
        for ln in f.readlines():
            inp.append(np.array(list(map(int, ln.split()))))
    return inp


def gendeps(a):
    res = [a]
    while not all(map(lambda x: x == 0, res[-1])):
        crnt = res[-1][1:] - res[-1][:-1]
        res.append(crnt)

    return res


def previous(i):
    def get_next(k): return [x[0] for x in gendeps(k)]
    prev = get_next(i)[::-1]
    res = [0] * len(prev)
    for i in range(1, len(prev)):
        res[i] = prev[i-1] - res[i-1]
    return prev[-1] - res[-1]


def part1(fname: str, parse_func) -> int:
    instr = parse_func(fname)
    def get_next(k): return sum([x[-1] for x in gendeps(k)])
    return sum([get_next(k) for k in instr])


def part2(fname: str, parse_func) -> int:
    instr = parse_func(fname)
    return sum([previous(k) for k in instr])


def main():
    print("Part 1: ", part1("test", parse_inut))
    print("Part 1: ", part1("input", parse_inut))
    print("Part 2: ", part2("test", parse_inut))
    print("Part 2: ", part2("input", parse_inut))


main()
