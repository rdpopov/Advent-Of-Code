import numpy as np
from collections import defaultdict
import math as m

def connectNodes(matr,connections,points) -> np.array(int):
    inf_val = matr[0,0] ## diagonals have max value
    graphs = {}
    components = 0

    while connections > 0:
        a = np.min(matr)
        minpair = np.argwhere(matr == a)
        i,j = minpair[0][0], minpair[0][1]
        # print("Connecting points ", points[i], " and ", points[j], " with distance ", a)
        if i in graphs and j in graphs:
            # print("Both nodes are already connected")
            if graphs[i] != graphs[j]:
                for g in graphs:
                    if graphs[g] == graphs[j]:
                        graphs[g] = graphs[i]
                connections -=1
        elif i in graphs:
            graphs[j] = graphs[i]
            connections -=1
        elif j in graphs:
            graphs[i] = graphs[j]
            connections -=1
        else:
            # print("Creating new component")
            graphs[j] = components
            graphs[i] = components
            components += 1
            connections -=1

        matr[i][j] = inf_val
        matr[j][i] = inf_val
    
    components_dict = defaultdict(int)
    for i in range(components):
        for g in graphs:
            if graphs[g] == i:
                components_dict[i] +=1
    vals = [components_dict[g] for g in components_dict]
    vals.sort(reverse=True)
    return m.prod(vals[:3])
    # print(str(components_dict))

def dst(p1,p2):
    return sum((p1 - p2) ** 2) ** 0.5

class pque():
    def __init__(self):
        self.distances = defaultdict(list)
        self.lenghts = None

    def add(self,id_p1,id_p2,d):
        self.distances[d].append((id_p1,id_p2))

    def prime_for_searching(self):
        self.lenghts = list(self.distances.keys())
        self.lenghts.sort()

    def get_smallest_pair(self):
        if self.lenghts == None:
            self.prime_for_searching()
        min_d = self.lenghts[0]
        pair = self.distances[min_d].pop(0)
        if len(self.distances[min_d]) == 0:
            del self.distances[min_d]
            self.lenghts.pop(0)
        return pair[0],pair[1],min_d
    def out(self):
        for d in self.distances:
            print(d,self.distances[d])

class connComps():
    def __init__(self):
        self.nodes = list()
        self.last_connected = None

    def find(self, node_id):
        for i,n in enumerate(self.nodes):
            if node_id in n:
                return i
        else:
            return -1

    def insert_conn(self,p1,p2) -> int:
        i1 = self.find(p1)
        i2 = self.find(p2)

        if i1 != -1 and i1 == i2:
            return 1
        elif i1 == -1 and i2 == -1:
            self.nodes.append(set([p1,p2]))
        elif i1 != -1 and i2 == -1:
            self.nodes[i1].add(p2)
        elif i1 == -1 and i2 != -1:
            self.nodes[i2].add(p1)
        elif i1 != -1 and i2 != -1:
            self.nodes[i1] = self.nodes[i1].union(self.nodes[i2])
            self.nodes.pop(i2)
        self.last_connected = (p1,p2)
        return 1

    def calculate_Part1(self) -> int:
        # for n in self.nodes:
            # print(n)
        lst  = [len(s) for s in self.nodes] #+ [1,1,1]
        lst.sort(reverse=True)
        print(lst)
        return m.prod(lst[:3])
    
    def calculate_Part2(self,nodesCoord) -> int:
        p1 = nodesCoord[self.last_connected[0]]
        p2 = nodesCoord[self.last_connected[1]]
        print("last_connected",p1,p2)
        return p1[0] * p2[0]

    def allConnected(self) -> int:
        if self.last_connected is None:
            return 0
        return len(self.nodes[0])


def connNodes(que: pque, nconn) -> connComps:
    comps = connComps()
    while nconn > 0:
        p1,p2,d = que.get_smallest_pair()
        res = comps.insert_conn(p1,p2)
        nconn -= res
    return comps

def connAllNodes(que: pque, nnodes) -> connComps:
    comps = connComps()
    while nnodes > comps.allConnected():
        p1,p2,d = que.get_smallest_pair()
        comps.insert_conn(p1,p2)
    return comps


def part1(file: str,connections:int) -> int:
    res = 0
    maximal = 0
    with open(file, 'r') as f:
        lines = np.array([np.array(list(map(int,line.split(',')))) for line in f])
        que = pque()

        for i in range(len(lines)):
            for j in range(i,len(lines)):
                if i!=j:
                    d = dst(lines[i],lines[j])
                    que.add(i,j,d)

        # que.out()
        res = connNodes(que,connections).calculate_Part1()

        return file,res


def part2(file: str) -> int:
    res = 0
    maximal = 0
    with open(file, 'r') as f:
        lines = np.array([np.array(list(map(int,line.split(',')))) for line in f])
        que = pque()

        for i in range(len(lines)):
            for j in range(i,len(lines)):
                if i!=j:
                    d = dst(lines[i],lines[j])
                    que.add(i,j,d)

        # que.out()
        res = connAllNodes(que,len(lines)).calculate_Part2(lines)

        return file,res

def main():
    print( part1('ex',10))
    print( part1('input',1000))

    print( part2('ex'))
    print( part2('input'))

if __name__ == "__main__":
    main()
