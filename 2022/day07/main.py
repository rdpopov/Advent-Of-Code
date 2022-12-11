#!/usr/bin/python3

def getRefCrntRoot(dct,path): 
    if len(path) == 1:
        if path[0] not in dct: # dont think this will happen, we know folders before they are acessed
            # print("added",path[0])
            dct[path[0]] = {}
        return dct[path[0]]
    return getRefCrntRoot(dct[path[0]],path[1:])


def print_tree(dct,indent = "  "):
    for i in dct:
        if isinstance(dct[i],dict):
            print(indent,i)
            print_tree(dct[i],indent + "  ")
        else:
            print(indent,i,dct[i])

def calculate_sum_sizes(dct):
    sum_node = 0
    for i in dct:
        if isinstance(dct[i],dict):
            sum_node += calculate_sum_sizes(dct[i])
        else:
            sum_node += dct[i]
    dct["size ->"] = sum_node
    return sum_node

def sum_tree_above_x(dct,x):
    sum_node = 0
    for i in dct:
        if isinstance(dct[i],dict):
            sum_node += sum_tree_above_x(dct[i],x)
    if dct["size ->"]<x:
        sum_node += dct["size ->"]
    return sum_node

def find_smallest_bigger(dct,x):
    in_children = []
    if dct['size ->'] > x:
        in_children.append(dct['size ->'])
    for i in dct:
        if isinstance(dct[i],dict):
            in_children.extend(find_smallest_bigger(dct[i],x))
    return in_children

def collect_input(fname):
    dirs = {}
    crntPath = []
    crntRoot = None
    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            if line[0] == '$': # we are in a command
                if line[1] == 'cd':
                    if line[2] == '..':
                        crntPath.pop(-1)
                        # current root ref would have to be recalculated
                        # so now we just 'invalidate' it and get it when needed
                        # adding .. as a upper ref might add cycles
                        # all cd commands jum only one folder at a a time
                    else:
                        crntPath.append(line[2])
                elif line[1] == 'ls':
                    # ls command kind of useless now
                    pass
            else:
                crntRoot = getRefCrntRoot(dirs, crntPath)
                if line[0] == 'dir':
                    getRefCrntRoot(dirs, crntPath + [line[1]])
                    # i guss that works to add the missing path
                else:
                    crntRoot["file "+line[1]] = int(line[0])
                    # no colisions
    calculate_sum_sizes(dirs)
    return dirs

def scenario1(fname):
    dirs = collect_input(fname)
    print(sum_tree_above_x(dirs,100000))


def scenario2(fname):
    dirs = collect_input(fname)
    fs_size = 70000000
    update_size = 30000000
    needed_size = update_size - (fs_size - dirs["size ->"])
    print(min(find_smallest_bigger(dirs,needed_size)))


if __name__ == "__main__":
    scenario1("./input1")
    scenario1("./input")
    scenario2("./input1")
    scenario2("./input")
