import numpy as np

def countFree(grid, y, x):
    count = 0
    for i in range(-1,2):
        for j in range(-1,2):
            if j !=0 or i !=0:
                if x+i < 0 or y+j<0:
                    pass
                elif x+i >=len(grid[0]) or y+j >= len(grid):
                    pass
                else:
                    if grid[y+j][x+i] == '@':
                        count +=1
    return count

def positionsToFree(grid)-> list[tuple[int,int]]:
    res = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == '@':
                if countFree(grid,y,x) <4:
                    res.append((y,x))
    return res

def freePosiotions(grid,l)-> list[tuple[int,int]]:
    for tp in l:
        y,x = tp
        grid[y][x] = '.'

def part1(file: str) -> int:
    with open(file, 'r') as f:
        lines = [list(line.rstrip()) for line in f]
        res =len(positionsToFree(lines))

        return file,res

def part2(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [list(line.rstrip()) for line in f]
        while True:
            to_free = positionsToFree(lines)
            if len(to_free) == 0:
                return file,res
            res += len(to_free)
            freePosiotions(lines,to_free)

def main():
    # print( part1('ex'))
    # print( part1('input'))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
