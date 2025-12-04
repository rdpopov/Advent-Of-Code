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
            rot = int(line[1:]) * mul
            prv = pos
            pos += rot
            if sign(prv) + sign(pos) == 0:
                res += (abs(rot) // 100 + 1)
            elif abs(pos) >= 100:
                res += abs(pos //100)
            pos = pos % 100

            # print(pos)
            if pos == 0:
                # print("zero", line , pos ,rot) 
                res +=1
    return file,res

def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
