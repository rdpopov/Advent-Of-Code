#!/usr/bin/python3

def gen_new_board(size):
    return [[0 for i in range(size+2) ] for j in range(size+2)  ] 

def next_state(main,i,j):
    on = 0
    for i_ in range(i-1,i+2):
        for j_ in range(j-1,j+2):
            on += main[i_][j_]
    on = on - main[i][j] # remove the value of crnt cell from total

    if on == 3 or on == 2 and main[i][j] == 1:
        return 1
    if on == 3 and main[i][j] == 0:
        return 1
    return 0

def nex_state_board_s1(main,spare,size):
    for i in range(1,size+1):
        for j in range(1,size+1):
            spare[i][j] = next_state(main,i,j)

def nex_state_board_s2(main,spare,size):
    for i in range(1,size+1):
        for j in range(1,size+1):
            spare[i][j] = next_state(main,i,j)
    spare[1][1]           = 1
    spare[1][size]      = 1
    spare[size][1]      = 1
    spare[size][size] = 1


def print_board(main,size):
    for i in range(1,size+1):
        for j in range(1,size+1):
            print('#' if main[i][j] == 1 else '.',end="")
        print()
    print()

def count_living(main,size):
    s = 0
    for i in range(1,size+1):
        for j in range(1,size+1):
            s +=  main[i][j]
    return s


def sceanrio1():
    board_size = 100
    iterations = 100
    main = gen_new_board(board_size)
    spare = gen_new_board(board_size)

    with open("./input") as f:
        for i,line in zip(range(1,board_size+1),f.readlines()):
            l=line.strip()
            for j,c in zip(range(1,board_size+1),l):
                if c == "#":
                   main[i][j] = 1

    for i in range(iterations):
        #print_board(main,board_size)
        nex_state_board_s1(main,spare,board_size)
        main, spare = spare,main

    print(count_living(main,board_size))


def sceanrio2():
    board_size = 100
    iterations = 100
    main = gen_new_board(board_size)
    spare = gen_new_board(board_size)

    with open("./input") as f:
        for i,line in zip(range(1,board_size+1),f.readlines()):
            l=line.strip()
            for j,c in zip(range(1,board_size+1),l):
                if c == "#":
                   main[i][j] = 1
    main[1][1] = 1
    main[1][board_size] = 1
    main[board_size][1] = 1
    main[board_size][board_size] = 1

    for i in range(iterations):
        # print_board(main,board_size)
        nex_state_board_s2(main,spare,board_size)
        main, spare = spare,main

    print(count_living(main,board_size))





if __name__ == "__main__":
    #sceanrio1()
    sceanrio2()
