#!/usr/bin/python3
import json

def gen_perm(lst,MAX_SEATS):
    for i in range(MAX_SEATS):
        if i not in lst:
            yield from gen_perm(lst + [i],MAX_SEATS)
    if len(lst) == MAX_SEATS:
        yield lst


def calc_hapiness(grf,perm,seating,MAX_SEATS):
    cost = 0
    for i in range(MAX_SEATS):
        prv = seating[perm[i-1]]
        nxt = seating[perm[(i+1) % MAX_SEATS]]
        crnt = seating[perm[i]]
        cost= cost + grf[crnt][nxt] + grf[crnt][prv]
    return cost


def addToDict(grf,seating, fr, to, cost):
    if fr in grf:
        grf[fr][to] = cost
    else:
        seating.append(fr)
        grf[fr] = {to: cost}


def sceanrio1():
    sum = 0
    grf = {}
    seating= []
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-2].split()
            fr = line[0]
            to = line[-1]
            cost = line[3]
            cost_dir = line[2]
            if cost_dir == "lose":
                addToDict(grf,seating,fr,to, -int(cost))
            else:
                addToDict(grf,seating,fr,to, int(cost))
            MAX_SEATS = len(grf.keys())
    # print(json.dumps(grf,indent=2))
    
    max_perm = [i for i in range(MAX_SEATS)]
    max_happy = calc_hapiness(grf,max_perm,seating,MAX_SEATS)
    for perm in gen_perm([],MAX_SEATS):
        max_happy,max_perm = max( (calc_hapiness(grf,perm,seating,MAX_SEATS),perm),
                (max_happy,max_perm), key=lambda x: x[0])
    print(max_happy,max_perm)
    return (max_happy,max_perm)

def painless_insert(grf,perm,seating,MAX_SEATS):

    prv = seating[perm[0]]
    nxt = seating[perm[-1]]
    diff = grf[prv][nxt] + grf[nxt][prv]
    for p,n in zip(perm[:-1],perm[1:]):
        prv = seating[p]
        nxt = seating[n]
        diff = min(grf[prv][nxt] + grf[nxt][prv],diff)
    return diff



def sceanrio2():
    sum = 0
    grf = {}
    seating= []
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-2].split()
            fr = line[0]
            to = line[-1]
            cost = line[3]
            cost_dir = line[2]
            if cost_dir == "lose":
                addToDict(grf,seating,fr,to, -int(cost))
            else:
                addToDict(grf,seating,fr,to, int(cost))
            MAX_SEATS = len(grf.keys())
    # print(json.dumps(grf,indent=2))
    
    max_perm = [i for i in range(MAX_SEATS)]
    max_happy = calc_hapiness(grf,max_perm,seating,MAX_SEATS)
    for perm in gen_perm([],MAX_SEATS):
        max_happy,max_perm = max( (calc_hapiness(grf,perm,seating,MAX_SEATS),perm),
                (max_happy,max_perm), key=lambda x: x[0])
    print(max_happy - painless_insert(grf,max_perm,seating,MAX_SEATS))
    return max_happy - painless_insert(grf,max_perm,seating,MAX_SEATS)


if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
