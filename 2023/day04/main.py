
def score_part1(win: list, own: list) -> int:
    ret = 0
    for i in own:
        if i in win:
            ret += 1
    if ret > 0:
        return 1 << (ret - 1)
    return 0


def score_part2(win: list, own: list) -> int:
    ret = 0
    for i in own:
        if i in win:
            ret += 1
    return ret


def part1(fname: str):
    res = 0
    with open(fname) as f:
        for ln in f.readlines():
            _, nums = ln.split(':')
            winning, own = map(lambda x: list(
                map(int, x.split())), nums.split('|'))
            # print(winning, own)
            res += score_part1(winning, own)
    return res


def part2(fname: str):
    cards = []
    with open(fname) as f:
        for ln in f.readlines():
            # id, nums = ln.split(':')
            _, nums = ln.split(':')
            # id = int(id.split()[1])
            winning, own = map(lambda x: list(
                map(int, x.split())), nums.split('|'))
            cards.append([score_part2(winning, own), 1])
    max_cards = len(cards)
    for i in range(max_cards):
        for iprm in range(i+1, min(i+cards[i][0]+1, max_cards)):
            cards[iprm][1] += cards[i][1]
    # print(cards)

    return sum(map(lambda x: x[1], cards))


def main():
    print("Part 1 test", part1("test"))
    print("Part 1 input", part1("input"))
    print("Part 2 test", part2("test"))
    print("Part 2 input", part2("input"))


main()
