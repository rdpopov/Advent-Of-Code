import numpy as np
from itertools import combinations
from collections import defaultdict
import math as m


# clean up buttons
def toTurnOn(lst:list,len):
    res = [0]*len
    for i in lst:
        res[i] = 1
    return np.array(res)

def tupToTurnOn(tup,len):
    tup = list(map(int,tup[1:-1].split(',')))
    return toTurnOn(tup,len)

# clean up lhs
def lhsToNp(lhs:str):
    return np.array([ {'#': 1, '.': 0 }[c] for c in lhs[1:-1]])

def lhs2ToNp(lhs:str):
    return list(map(int,lhs[1:-1].split(',')))

def GenSolutionVecs(crnt,limits):
    res = list()
    for i in range(crnt.shape[0]):
        willAdd = crnt.copy()
        willAdd[i]+=1
        print(i)
        if limits[i] == 0 or willAdd[i] < limits[i]:
            res.append((i,willAdd))
    return res

class eq:
    def __init__(self, lhs, ops, rhs):
        self.lhs = lhsToNp(lhs)
        self.ops = np.array(list(map(lambda x: tupToTurnOn(x,self.lhs.shape[0]),ops)))
        self.rhs = lhs2ToNp(rhs.strip())

    def __repr__(self):
        res = ""
        goal_1 = f"lhs{self.lhs}\n"
        vecs = "vecs:\n"
        for i,v in enumerate(self.ops):
            vecs += f"{i}: {v}\n"
        goal_2 = f"rhs{self.rhs}\n"
        return vecs + goal_2 + str(self.ops.shape)

    def Part1Solve(self):
        for i in range(1,len(self.ops)+1):
            for ops in combinations(self.ops, r=i):
                if (np.sum(np.array(ops), axis=0)%2 == self.lhs).all():
                    return i

    def Part2Solve(self):
        sol = np.array([0]*self.ops.shape[0])
        limits = self.rhs.copy()


        while 1:

            best = np.zeros(self.ops.shape[0])
            found = False
            for idx,tmpSol in GenSolutionVecs(sol,limits):
                tmpRes = tmpSol @ self.ops
                after  = self.rhs - tmpRes
                if all(after == 0):
                    return sum(tmpSol)
                if all(after > 0):
                    found = True
                    if sum(after) < sum(self.rhs - (best @ self.ops)):
                        best = tmpSol
                else:
                    limits[idx] = tmpSol[idx]-1

            if found:
                sol = best
            else:
                sol = np.zeros(self.ops.shape[0])

        return sum(sol)



def part1(file: str) -> int:
    res = 0 
    with open(file, 'r') as f:
        lines = [line.split(' ') for line in f]

        for line in lines:
            eqs = eq(line[0], line[1:-1], line[-1])
            # print(eqs)
            res += eqs.Part1Solve()
            # return res

        return file,res

def part2(file: str) -> int:
    res = 0
    with open(file, 'r') as f:
        lines = [line.split(' ') for line in f]

        for i,line in enumerate(lines):
            eqs = eq(line[0], line[1:-1], line[-1])
            # print(eqs)
            current = eqs.Part2Solve()

            print(f"Line {i}: {current}")

            res += current
            

        return file,res

def main():
    print( part1('ex'))
    # print( part1('input'))

    print( part2('ex'))
    # print( part2('input'))

if __name__ == "__main__":
    main()
