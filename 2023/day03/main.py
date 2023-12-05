import sys


def is_num(x):
    return x in '1234567890.'

def is_not_near_symbol(matr: list[list], row: int, sl: tuple) -> bool:
    for x in range(sl[0]-1, sl[1]+1):
        if 0 <= x < len(matr):
            for dy in [-1, 0, 1]:
                y = row + dy
                if 0 <= y < len(matr):
                    if not is_num(matr[y][x]):
                        return False
    return True

def if_gear_get_id(matr: list[list],gears:dict, row: int, sl: tuple) -> tuple|None:
    for x in range(sl[0]-1, sl[1]+1):
        if 0 <= x < len(matr):
            for dy in [-1, 0, 1]:
                y = row + dy
                if 0 <= y < len(matr):
                    if matr[y][x] == '*':
                        return (y,x)
    return None

def parse_nums_in_line(row: list) -> list[tuple]:
    acc = 0
    start = 0
    res = []
    for i in range(len(row)):
        if '0' <= row[i] <= '9':
            if acc == 0:
                start = i
            acc = (acc * 10) + int(row[i])
        else:
            if acc != 0:
                res.append((acc, (start, i)))
                acc = 0
    if acc > 0:
        res.append((acc, (start, len(row))))
    return res


def part1(fname: str) -> int:
    matr = []
    res = 0
    with open(fname) as inp:
        for i in inp.readlines():
            matr.append(i)

    for j in range(len(matr)):
        for (n, r) in parse_nums_in_line(matr[j]):
            res += n
            if is_not_near_symbol(matr, j, r):
                res -= n

    return res

def part2(fname: str) -> int:
    matr = []
    gears = {}
    res = 0
    with open(fname) as inp:
        for i in inp.readlines():
            matr.append(i)

    for j in range(len(matr)):
        for i in range(len(matr[j])):
            if matr[j][i] == '*':
                gears[(j,i)] = []

    for j in range(len(matr)):
        for (n, r) in parse_nums_in_line(matr[j]):
            maybe_gear = if_gear_get_id(matr, gears, j, r)
            if maybe_gear:
                gears[maybe_gear].append(n)

    for k, v in gears.items():
        if len(v) == 2:
            res += v[0]*v[1]
    return res

def main():
    print("Part 1 test: ", part1("test"))
    print("Part 1 input: ", part1("input"))
    print("Part 2 test: ", part2("test"))
    print("Part 2 input: ", part2("input"))

main()
