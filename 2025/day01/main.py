def part1(file: str) -> int:
    pos = 50
    res = 0
    with open(file, 'r') as f:
        data = f.read().strip().splitlines()
        for line in data:
            mul = 1 
            if line [0] == 'L':
                mul = -1
            pos = (pos + int(line[1:]) * mul) % 100
            # print(pos)
            if pos == 0:
                res +=1
    return file,res

def sign(x: int) -> int:
    if x < 0:
        return -1
    return 1

def part2(file: str) -> int:
    pos = 50
    res = 0
    with open(file, 'r') as f:
        data = f.read().strip().splitlines()
        for line in data:
            mul = 1 
            if line [0] == 'L':
                mul = -1
            rot = int(line[1:])
            res += rot // 100
            rot %= 100
            rot *= mul
            # print(f"{line} rot:{rot} ", end=' ')
            # print(pos + rot ,end=' ')
            if (pos != 0):
                if (pos + rot > 100) or (pos + rot < 0):
                    # print("0->\t",line,end=' ')
                    res += 1
            pos = (pos + rot) % 100

            if pos == 0:
                res +=1
    return file,res

def main():
    # print( part1('ex'))
    # print( part1('input'))

    # print( part2('ex2'))
    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
