#!/usr/bin/python3
import json
def prs(lst):
    subseq = [[lst[0]]]
    ret = []
    for i in lst[1:]:
        if i == subseq[-1][-1]:
            subseq[-1].append(i)
        else:
            subseq.append([i])
    for i in subseq:
        ret.append(len(i))
        ret.append(i[0])
    return ret

def sceanrio1():
    iters = 40
    lst = [1,3,2,1,1,3,1,1,1,2] 
    # lst = [1]
    for i in range(iters):
        lst = prs(lst)
    # print(lst)
    print(len(lst))

def sceanrio2():
    iters = 50
    lst = [1,3,2,1,1,3,1,1,1,2] 
    # lst = [1]
    for i in range(iters):
        lst = prs(lst)
    # print(lst)
    print(len(lst))

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()


