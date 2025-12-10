import numpy as np

def isFresh(product: int, ranges: list[list[int]]) -> int:
    for r in ranges:
        if r[0] <= product <= r[1]:
            return 1
    return 0


def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [line.rstrip() for line in f]
        index_empty  = lines.index('')
        ranges = [list(map(int,r.split('-')))  for r in lines[:index_empty]]
        products = [int(p) for p in lines[index_empty+1:]]
        res = sum(map(lambda p: isFresh(p,ranges),products))

        return file,res

def collapseRanges(ranges):
    collapsed = True
    while collapsed:
        collapsed = False
        for i in range(len(ranges)):
            for j in range(len(ranges)):
                if i!=j:
                    if ranges[i][0] <= ranges[j][0] <= ranges[i][1]:
                        ranges[i][1] = max(ranges[i][1], ranges[j][1])
                        ranges[j] = [-1,-1]
                        collapsed = True
        while [-1,-1] in ranges:
            ranges.remove([-1,-1])


def part2(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [line.rstrip() for line in f]
        index_empty  = lines.index('')
        ranges = [list(map(int,r.split('-')))  for r in lines[:index_empty]]
        products = [int(p) for p in lines[index_empty+1:]]

        collapseRanges(ranges)
        for r in ranges:
            res += r[1] - r[0] + 1

        return file,res


def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
