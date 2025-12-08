def is_bad(n):
    s = str(n)
    ls = len(s)
    return ls%2 == 0 and s[:ls//2] == s[ls//2:]

def to_Parts(s,n):
    return [s[i:i + n] for i in range(0, len(s), n)]

def allSame (lst):
    return all(x == lst[0] for x in lst)

def is_bad_part2(n):
    s = str(n)
    ls = len(s)
    for i in range(1,ls):
        if allSame(to_Parts(s,i)) and ls%i == 0:
            return True
    return False


def get_bad_ranges(r,bad):
    res = []
    for i in range(r[0],r[1]+1):
        if bad(i):
            res.append(i)
    return res


def part1(file: str) -> int:
    cnt = 0 
    with open(file, 'r') as f:
        lines = f.readlines()
        line = lines[0].strip()
        line = list(map(lambda x: list(map(int ,x.split('-'))), line.split(',')))
        for r in line:
            res = get_bad_ranges(r,is_bad)
            cnt += sum(res)
    return file,cnt

def part2(file: str) -> int:
    cnt = 0 
    with open(file, 'r') as f:
        lines = f.readlines()
        line = lines[0].strip()
        line = list(map(lambda x: list(map(int ,x.split('-'))), line.split(',')))
        for r in line:
            res = get_bad_ranges(r, is_bad_part2)
            cnt += sum(res)
    return file,cnt

def main():
    print( part1('ex'))
    print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
