import numpy as np
import math as m

def transpose(matrix):
    return list(map(list, zip(*matrix)))

def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [line.rstrip() for line in f]
        for i in range(len(lines)):
            if '*' not in lines[i]:
                lines[i] = list(map(int, lines[i].split()))
            else:
                lines[i] = lines[i].split()
        transposed = transpose(lines)
        for i in transposed:
            if '*' in i:
                res += m.prod(i[:-1])
            elif '+' in i:
                res += sum(i[:-1])

        return file,res

def applyOp(acc, num, op):
    if op == '+':
        return acc + num
    if op == '*':
        if acc == 0:
            return num
        return acc * num

def part2(file: str) -> int:
    with open(file, 'r') as f:
        lines = [line for line in f]
        lines = transpose(lines)
        last_op = None
        res = 0
        acc = 0
        for i in range(len(lines)-1):
            if lines[i][-1] != ' ':
                last_op = lines[i][-1]
                res += acc
                acc = 0

            num = ''.join(lines[i][:-1]).strip()
            if num != '':
                num = int(num)
                acc = applyOp(acc, num, last_op)
        res += acc

        return file,res


def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
