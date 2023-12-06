from functools import partial


def range_interp(to , frm, size, number):
    if frm <= number < (frm + size):
        return "", to + (number - frm)
    return None, number

def parse_inut(fname):
    input_text = []
    with open(fname) as f:
        tmp = []
        for i in f.readlines():
            if len(i) == 1:  # new line
                input_text.append(tmp)
                tmp = []
            else:
                tmp.append(i)

    seeds, transition = (input_text[0], input_text[1:])
    tables = []

    for trns in transition:
        fr_to, ranges = (trns[0], list(
            map(lambda x: map(int, x.split()), trns[1:])))
        frm, _, to = fr_to.split()[0].split('-')
        tables.append({
            "from": frm,
            "to": to,
            "ranges": list(map(lambda x: partial(range_interp, *x),
                               ranges))})
    seeds = list(map(int, seeds[0].split(':')[1].split()))
    return tables, seeds


def part1(fname: str) -> int:
    tables, seeds = parse_inut(fname)

    minseed = None
    for s in seeds:
        for t in tables:
            res_for_crnt = list(filter(lambda x: x[0] is not None, map(lambda x: x(s) ,t["ranges"])))
            if res_for_crnt:
                s = res_for_crnt[0][1]
        if not minseed:
            minseed = s
        minseed = min(minseed, s)
    return(minseed)

def explode_seeds (seeds):
    s = seeds
    ret = []
    while s:
        pr,s = s[0:2],s[2:]
        ret.append((pr[0],pr[0]+pr[1]))
    return ret


def part2(fname: str) -> int:
    tables, seeds = parse_inut(fname)
    seeds = explode_seeds(seeds)

    minseed = None
    for sr in seeds:
        for s in range(*sr):
            for t in tables:
                res_for_crnt = list(filter(lambda x: x[0] is not None, map(lambda x: x(s) ,t["ranges"])))
                if res_for_crnt:
                    s = res_for_crnt[0][1]
            if not minseed:
                minseed = s
            minseed = min(minseed, s)
    return minseed


def main():
    print("Part 1: ",part1("test"))
    print("Part 1: ",part1("input"))
    print("Part 2: ",part2("test"))
    print("Part 2: ",part2("input"))

main()

