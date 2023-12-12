from functools import partial
from functools import cmp_to_key
from collections import Counter

ord_p1 = "AKQJT98765432"
ord_p2 = "AKQT98765432J"


def quantify(part2, hand):
    V = Counter(hand)
    v = sorted(list(Counter(V).values()))
    if part2:
        if V['J']:
            tmp = V.pop('J')
            v = sorted(list(V.values())) or [0]
            v[-1] += tmp
    ret = 0
    if v == [5]:
        ret = 6
    elif v == [1, 4]:
        ret = 5
    elif v == [2, 3]:
        ret = 4
    elif v == [1, 1, 3]:
        ret = 3
    elif v == [1, 2, 2]:
        ret = 2
    elif v == [1, 1, 1, 2]:
        ret = 1
    elif v == [1, 1, 1, 1, 1]:
        ret = 0
    return ret


def cust_compare(ord, tup_hand1, tup_hand2):
    if tup_hand1[0] < tup_hand2[0]:
        return -1
    if tup_hand2[0] < tup_hand1[0]:
        return 1

    weights = dict([(v, i) for (i, v) in enumerate(ord[::-1])])
    for h1, h2 in zip(tup_hand1[1][0], tup_hand2[1][0]):
        if h1 == h2:
            continue
        elif weights[h1] < weights[h2]:
            return -1
        else:
            return 1
    return 0


def parse_inut(fname):
    inp = []
    with open(fname) as f:
        for i in f.readlines():
            k, v = i.split()
            inp.append((k, int(v)))
    return inp


def part1(fname: str, parse_func) -> int:
    inp = parse_func(fname)
    q = partial(quantify, False)
    quantified = list(map(lambda x: (q(x[0]), x), inp))
    quantified.sort(key=cmp_to_key(partial(cust_compare, ord_p1)))
    return sum(map(lambda x: (x[0]+1) * (x[1][1][1]), enumerate(quantified)))


def part2(fname: str, parse_func) -> int:
    inp = parse_func(fname)
    q = partial(quantify, True)
    quantified = list(map(lambda x: (q(x[0]), x), inp))
    quantified.sort(key=cmp_to_key(partial(cust_compare, ord_p2)))
    return sum(map(lambda x: (x[0]+1) * (x[1][1][1]), enumerate(quantified)))


def main():
    print("Part 1: ", part1("test", parse_inut))
    print("Part 1: ", part1("input", parse_inut))
    print("Part 2: ", part2("test", parse_inut))
    print("Part 2: ", part2("input", parse_inut))


main()
