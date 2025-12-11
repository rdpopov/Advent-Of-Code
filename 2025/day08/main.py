import numpy as np
import math as m

def transform(l1,l2,matr):
    spl = 0
    for  i in  range(len(l2)-2):
        s1 = ''.join(l1[i:i+3])
        s2 = ''.join(l2[i:i+3])

        if s1[1] == '|' and s2[1] == '.':
            l2[i+1] = "|"

        if s1[1] == 'S' and s2[1] == '.':
            matr[i+1] = 1
            l2[i+1] = "|"

        if s1[1] == '|' and s2[1] == '^':
            spl +=1
            matr[i] += matr[i+1]
            matr[i+2] += matr[i+1]
            matr[i+1] = 0
            if s2[0] == '.':
                l2[i] = "|"
                spl +=1
            if s2[2] == '.':
                l2[i+2] = "|"
    return spl

def itr(field) -> int:
    spl = 0
    matr = np.zeros(len(field[0]))
    for i in range(len(field)):
        if i == 0 or i == len(field)-1:
            continue
        spl+=transform(field[i-1], field[i],matr)
    return spl,int(sum(matr))

def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [list(line.rstrip()) for line in f]
        res,_ = itr(lines)
        return file,res


def part2(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [list(line.rstrip()) for line in f]
        _,res = itr(lines)
        return file,res

def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
