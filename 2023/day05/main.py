from functools import partial


def range_interp(to , frm, size, number):
    if frm <= number < (frm + size):
        return "", to + (number - frm)
    return None, number

def parse_inut(fname, range_fn):
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
            "ranges": list(map(lambda x: partial(range_fn, *x),
                               ranges))})
    seeds = list(map(int, seeds[0].split(':')[1].split()))
    return tables, seeds


def part1(fname: str) -> int:
    tables, seeds = parse_inut(fname,range_interp)

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


def explode_seeds(seeds):
    s = seeds
    ret = []
    while s:
        pr, s = s[0:2], s[2:]
        ret.append(pr)
    return ret


def range_split(to, frm, size, lower, ln):
    frm_u = frm + size
    upper = lower + ln
    if frm <= lower < frm_u and frm <= upper <= frm_u:  # fully inside
        # print("1: ", to, frm, size, lower, ln, " ->", end=" ")
        lower_bound = lower - frm + to
        return [(lower_bound, ln)], []

    if frm <= lower < frm_u and lower < frm_u < upper:  # upper end out
        # print("2: ", to, frm, size, lower, ln, " ->", end=" ")
        lower_bound = lower - frm + to
        sz = frm_u - lower
        return [(lower_bound, sz)], [(lower_bound + sz, ln - sz)]

    if lower < frm and frm <= upper < frm_u:  # lower end out
        # print("3: ", to, frm, size, lower, ln, " ->", end=" ")
        sz = upper - frm
        return [(to, sz)], [(lower, ln - sz)]

    if lower < frm and frm_u < upper:  # function range is smaller
        # print("4: ", to, frm, size, lower, ln, " ->", end=" ")
        sz = frm - lower
        return [(to, to + size)], [(lower, sz), (frm_u, (ln - (sz + size)))]

    # print("5: ", to, frm, size, lower, ln, " ->", end=" ")
    return [], [(lower, ln)]  # no overlap


def part_range(fns, rang):
    done = []
    que = [rang]
    for x in fns:
        swp = []
        for r in que:
            dn, rest = x(*r)
            # print(dn ,rest)
            done.extend(dn)
            swp.extend(rest)
        que = swp
    done.extend(que)
    return done



def part2(fname: str) -> int:
    tables, seeds = parse_inut(fname,range_split)
    seeds = explode_seeds(seeds)

    minseed = None

    for t in tables:
        que = []
        for s in seeds:
            d = part_range(t["ranges"],s)
            que.extend(d)
        seeds = que
# why are there zeros i do not know , dont care either
    return min(filter(lambda x: x > 0 ,map(lambda x : x[0] , seeds)))

def main():
    print("Part 1: ",part1("test"))
    print("Part 1: ",part1("input"))
    print("Part 2: ",part2("test"))
    print("Part 2: ",part2("input"))

main()

