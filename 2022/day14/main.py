#!/usr/bin/python3

def make_line(board,p1,p2):
    if p1[0]  == p2[0]:
        for j in range(min(p1[1],p2[1]), max(p1[1],p2[1])):
            board[p1[0]][j] = 1
    elif p1[1] == p2[1]:
        for j in range(min(p1[0],p2[0]), max(p1[0],p2[0])):
            board[j][p1[1]] = 1

def addv(v,v1):
    return v[0]+v1[0],v[1] + v1[1]

def empty_at(board,tup):
    return board[tup[0]][tup[1]]

def new_pos(board,point):
    mvecs = [(0,1), (-1,1),(1,1)]
    while True:
        if empty_at(board,addv(point,mvecs[0])) == 0:
            point = addv(point,mvecs[0])
        elif empty_at(board,addv(point,mvecs[1])) == 0:
            point = addv(point,mvecs[1])
        elif empty_at(board,addv(point,mvecs[2])) == 0:
            point = addv(point,mvecs[2])
        else:
            return point,True
        if point[0] == 999 or point[1] == 999:
            return point,False

def emplace(board):
    start = [500,0]
    point,done = new_pos(board, start)
    if done:
        board[point[0]][point[1]] = 1
    return done

def scenario1(fname):
    board = 1000 * [1000 * [0]]
    sand = 0
    with open(fname) as f:
        for line in f.readlines():
            l = [list(map(int,p.split(","))) for p in line[:-1].split("->")]
            for (p1,p2) in zip(l[:-1],l[1:]):
                make_line(board, p1, p2)

    while emplace(board):
        sand += 1
        if sand == 100:
            exit ()
        print(sand)
    print(sand)

def scenario2(fname):
    pass

if __name__ == "__main__":
    # scenario1("./input1")
    scenario1("./input")
    # scenario2("./input1")
    # scenario2("./input")
