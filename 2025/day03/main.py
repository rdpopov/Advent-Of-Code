import numpy as np

def get_max_digit(s) -> str:
    return np.argmax([c for c in s])

def maxDouble(s):
    max_digit = get_max_digit(s)
    if max_digit == len(s)-1:
        max2 = get_max_digit(s[:-1])
        return s[max2] + s[max_digit]
    else:
        max2 = get_max_digit(s[max_digit+1:])
        return s[max_digit] + s[max_digit+1+max2]


def part1(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        for l in f.readlines():
            line = l.strip()
            digit = maxDouble(line)
            # print(digit)
            res += int(digit)

    return file,res


def main():
    print( part1('ex'))
    print( part1('input'))

if __name__ == "__main__":
    main()
