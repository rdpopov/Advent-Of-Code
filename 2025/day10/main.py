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

def GenSolutionVecs(crnt, backtrack:int):
    res = list()
    if backtrack == 0:
        for i in range(crnt.shape[0]):
            willAdd = crnt.copy()
            willAdd[i]+=1
            res.append(willAdd)
    else:
        for i in range(len(crnt)):
            if crnt[i] > backtrack:
                willAdd = crnt.copy()
                willAdd[i]-= backtrack
                res.append(willAdd)
    return res

class eq:
    def __init__(self, lhs, ops, rhs):
        self.lhs = lhsToNp(lhs)
        self.ops = np.array(list(map(lambda x: tupToTurnOn(x,self.lhs.shape[0]),ops)))
        self.rhs = lhs2ToNp(rhs.strip())

    def __repr__(self):
        return f"lhs{self.lhs}\nvecs:{' '.join(map(str,self.ops))}\n??:{self.rhs}"

    def Part1Solve(self):
        for i in range(1,len(self.ops)+1):
            for ops in combinations(self.ops, r=i):
                if (np.sum(np.array(ops), axis=0)%2 == self.lhs).all():
                    return i

    def Part2Solve(self):
        sol = np.zeros(self.ops.shape[0])
        past_solutions = set()
        backtrack = 0
        bmul = 1
    
        while 1:
            bestSol = sol.copy()
            
            for tmpSol in GenSolutionVecs(sol,backtrack*bmul):
                if str(tmpSol) not in past_solutions:
                    # print(tmpSol)
                    tmpRes = tmpSol @ self.ops          # result with current solution vector
                    after  = self.rhs - tmpRes          # difference to target
                    if sum(after) == 0:
                        return sum(tmpSol)
                    if sum(after) == sum(abs(after)):   # if all are positive
                        if sum(bestSol @ self.ops) < sum(tmpRes):
                            bestSol = tmpSol

            # break
            if (bestSol == sol).all():

                backtrack = 0
                sol = np.zeros(self.ops.shape[0])

                backtrack += 1
                bmul = 1
            if backtrack > sum(sol):
                # return 0
                return -1
            else:
                past_solutions.add(str(sol))
                print(sol)
                bmul = 0
                sol = bestSol

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
    # print( part1('ex'))
    # print( part1('input'))

    print( part2('ex2'))
    # print( part2('input'))

if __name__ == "__main__":
    main()
