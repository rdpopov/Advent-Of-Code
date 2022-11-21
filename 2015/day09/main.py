#!/usr/bin/python3
import json
SOME_BIG_NR = 100000000 # could be something else
input_file = "./input" 

def findroute(grf,path):
    for dst in grf[path[-1]]:
        if dst not in path: # if destination is in path then we are done we are at the nth+1 step where n is number of nodes
            yield from findroute(grf,path + [dst])
    if len(path) == len(grf): # number of keys in graph == lenth of quinque destinations in path
        # print(path)
        yield path

def calculate(grf,path):
    sum = 0
    for (f,t) in zip(path[:-1],path[1:]):
        
        sum += int(grf[f][t])
    return sum

def findroute_wrap_min(grf):
    minpath = SOME_BIG_NR
    for st in grf:
        for i in grf[st]:
            for finished_path in findroute(grf,[st,i]):
                minpath = min(minpath,calculate(grf,finished_path))
    return minpath

def findroute_wrap_max(grf):
    maxpath = 0
    for st in grf:
        for i in grf[st]:
            for finished_path in findroute(grf,[st,i]):
                maxpath = max(maxpath,calculate(grf,finished_path))
    return maxpath

def addToDict(grf, fr, to, dt):
    if fr in grf:
        grf[fr][to] = dt
    else:
        grf[fr] = {to : dt}

def sceanrio1():
    sum = 0
    grf = {}
    nodes = []
    with open(input_file) as f:
        for i in f.readlines():
            line = i.strip().split()
            fr = line[0]
            to = line[2]
            dt = line[4]
            addToDict(grf,to,fr,dt)
            addToDict(grf,fr,to,dt)
        print(findroute_wrap_min(grf))

def sceanrio2():
    sum = 0
    grf = {}
    nodes = []
    with open(input_file) as f:
        for i in f.readlines():
            line = i.strip().split()
            fr = line[0]
            to = line[2]
            dt = line[4]
            addToDict(grf,to,fr,dt)
            addToDict(grf,fr,to,dt)
        print(findroute_wrap_max(grf))

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()


