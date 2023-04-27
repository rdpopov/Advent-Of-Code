#!/bin/bash
from hashlib import md5

move = [
         {"dir": 'U', "poss" : lambda xy: (xy[0]-1) >= 0 , "move": lambda xy:('U',(xy[0] - 1,xy[1])) },
         {"dir": 'D', "poss" : lambda xy: (xy[0]+1) < 4 , "move": lambda xy:('D',(xy[0] + 1,xy[1])) },
         {"dir": 'L', "poss" : lambda xy: (xy[1]-1) >= 0 , "move": lambda xy:('L',(xy[0],xy[1] - 1)) },
         {"dir": 'R', "poss" : lambda xy: (xy[1]+1) < 4 , "move": lambda xy:('R',(xy[0],xy[1] + 1)) }
         ]

def hashStr(s):
    # print(md5(s.encode()).hexdigest())
    return md5(s.encode()).hexdigest()[:4]

def traverse(coords,path:str,depth:int):
    # print(path,coords[0],coords[1])
    if coords == (3,3):
        yield path

    # print(coords,path)
    if depth > 0:
        # print(list(zip(hashStr(path),move)))
        new = [ m["move"](coords) for c,m in zip(hashStr(path),move) if m["poss"](coords) and c in "bcdef"]
        # print(new)
        for i in new:
            yield from traverse(i[1], path+i[0], depth-1)

def traverse_iterative_deepening(coords,path:str,depth:int):
    if coords != (3,3):
        if depth != 0:
            # print(list(zip(hashStr(path),move)))
            new = [ m["move"](coords) for c,m in zip(hashStr(path),move) if m["poss"](coords) and c in "bcdef"]
            # print(new)
            for i in new:
                yield from traverse_iterative_deepening(i[1], path+i[0], depth-1)
        else:
            yield (coords,path)


def run(pith):
    res = ""
    for i in traverse((0,0),pith,100):
        if len(i) < len(res) or len(res) == 0:
            res = i
    return res[len(pith):]

def run_part2(pith,depth_size):
    res = ""
    paths = [((0,0),pith)]
    iteration = 0 
    while 1:
        current_depth = []
        for i in paths:
            current_depth.extend(list(traverse_iterative_deepening(i[0], i[1], depth_size)))
        # print("iteration {} current_depth {}".format(iteration,len(current_depth)))
        if len(current_depth) == 0:
            res_list = []
            for i in paths:
                res_list.extend(list(traverse(i[0], i[1], depth_size)))
            for i in res_list:
                if len(res) < len(i):
                    res = i
            break
        else:
            paths = current_depth

        iteration+=1
    return len(res) - len(pith)

print("part 1",run("qtetzkpl"))
print("part 1",run_part2("qtetzkpl",100))

