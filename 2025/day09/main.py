import numpy as np
from collections import defaultdict
from itertools import combinations
import math as m
import shapely

def part1(file: str) -> int:
    maximal = 0
    res = 0 
    with open(file, 'r') as f:
        lines = np.array([np.array(list(map(int,line.split(',')))) for line in f])

        for i in lines:
            for j in lines:
                maximal = max(maximal, m.prod(abs(i-j)+1))
        res = maximal

        return file,res


def part2(file: str) -> int:
    res = 0 
    with open(file, 'r') as f:
        _,bound =part1(file)
        lines = np.array([tuple(map(int,line.split(','))) for line in f])
        polygon = shapely.Polygon(lines)
        for p1, p2 in combinations(lines, 2):
            x_min, x_max = min(p1[0], p2[0]), max(p1[0], p2[0])
            y_min, y_max = min(p1[1], p2[1]), max(p1[1], p2[1])

            area = (x_max - x_min + 1) * (y_max - y_min + 1)

            if polygon.contains(shapely.box(x_min, y_min, x_max, y_max)):
                res = max(res, area)
        return file,res

def main():
    # # print( part1('ex'))
    print( part1('input'))

    # print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
