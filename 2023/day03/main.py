import sys


def is_num(x):
    return x == '.' and '0' <= x <= '9'

def part1(fname: str, marble_count: dict) -> int:
    matr = []

    with open(fname) as inp:
        for i in inp.readlines():
            matr.append(i)

    for j in range(len(matr)):
        for i in range(len(matr[j])):
            s = matr[j][i]
            if not is_num(s):

    return 0


def main():
    print(part1("test"))
    print(part1("input"))

main()
