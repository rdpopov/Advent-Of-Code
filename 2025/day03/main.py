import numpy as np

def get_max_digit(s:str) -> int:
    return np.argmax([c for c in s])

def get_max_digit_absolute(s:str,low,high) -> int:
    return np.argmax([c for c in s[low:high]]) + low

def maxDouble(s):
    max_digit = get_max_digit(s)
    if max_digit == len(s)-1:
        max2 = get_max_digit(s[:-1])
        return s[max2] + s[max_digit]
    else:
        max2 = get_max_digit(s[max_digit+1:])
        return s[max_digit] + s[max_digit+1+max2]

def is_bigger_with(s,pivots,idx1,idx2):
    l1 = ''.join([s[i] for i in sorted(pivots + [idx1])])
    l2 = ''.join([s[i] for i in sorted(pivots + [idx2])])
    if l1 > l2:
        return idx1
    return idx2


def maxNDigits(s,n):
    pivots = [ get_max_digit_absolute(s,0,len(s))]
    while len(pivots) < n:
        current_best = -1
        for l,h in zip([-1] + pivots,pivots + [len(s)]):
            l +=1 
            if l >= h:
                continue
            crnt = get_max_digit_absolute(s,l,h)
            if current_best == -1 or is_bigger_with(s,pivots,crnt,current_best) == crnt:
                if crnt not in pivots:
                    current_best = crnt
        if current_best == -1:
            raise ValueError("Could not find next best digit")

        pivots.append(current_best)
        pivots = sorted(pivots)


    res = ''.join([s[p] for p in pivots])
    # print(res)
    return res

def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        for l in f.readlines():
            line = l.strip()
            digit =  maxNDigits(line,2)
            # print(digit)
            res += int(digit)

    return file,res

def part2(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        for l in f.readlines():
            line = l.strip()
            digit = maxNDigits(line,12)
            # print(digit)
            res += int(digit)

    return file,res


def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
