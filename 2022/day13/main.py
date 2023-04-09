#!/usr/bin/python3
from math import lcm

def compare(lst0,lst1):
    state = 0
    iterated = False
    # print("\tcomparing",lst0,lst1)
    for l0,l1 in zip(lst0,lst1):
        iterated = True
        if isinstance(l0,int) and isinstance(l1,int):
            if l0 < l1:
                return 1
            if l0 == l1:
                continue
            if l0 > l1:
                return -1
            # print("comparing",l0,l1,state)
        elif isinstance(l0,list) and isinstance(l1,int):
            state = compare(l0,[l1])
        elif isinstance(l0,int) and isinstance(l1,list):
            state = compare([l0],l1)
        elif isinstance(l0,list) and isinstance(l1,list):
            state = compare(l0,l1)
        if state != 0:
            break
    if not iterated:
        if len(lst0) < len(lst1):
            return 1
        return -1
    else:
        if state == 0 and len(lst0) <= len(lst1):
            return 1
        return state

def scenario1(fname):
    lst = []
    lstPairIdx = 1
    result = 0
    with open(fname) as f:
        for line in f.readlines():
            l = line[:-1]
            if len(l) > 0:
                # print(eval(l))
                lst.append(eval(l))
            if len(lst) == 2:
                res = compare(lst[0], lst[1])
                print("comparing",lstPairIdx,res,lst[0],lst[1])
                if res > 0:
                    result += lstPairIdx
                lst = []
                lstPairIdx += 1
    print(lst)
    print(result)

def scenario2(fname):
    pass

if __name__ == "__main__":
    scenario1("./input1")
    scenario1("./input")
    # scenario2("./input1")
    # scenario2("./input")
