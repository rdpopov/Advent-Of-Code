import numpy as np
import math as m


def transform(l1,l2):
    spl = 0
    for  i in  range(len(l2)-2):
        s1 = ''.join(l1[i:i+3])
        s2 = ''.join(l2[i:i+3])

        if s1[1] == '|' and s2[1] == '.':
            l2[i+1] = "|"

        if s1[1] == 'S' and s2[1] == '.':
            l2[i+1] = "|"

        if s1[1] == '|' and s2[1] == '^':
            if s2[0] == '.':
                l2[i] = "|"
                spl +=1
            if s2[2] == '.':
                l2[i+2] = "|"
                spl +=1


    return spl


def itr(field) -> int:
    for i in range(len(field)):
    # for i in range(len(field)):
        if i == 0 or i == len(field)-1:
            continue
        transform(field[i-1], field[i])
        # print(''.join(field[i]),s)
    spl = 0
    for i in range(len(field)-1):
        for j in range(len(field[i])):
            if field[i][j] == '|' and field[i+1][j] == '^':
                spl += 1
    return spl
    

def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [list(line.rstrip()) for line in f]
        res = itr(lines)
        
        return file,res

def main():
    print( part1('ex'))
    print( part1('input'))

if __name__ == "__main__":
    main()
