#!/usr/bin/python3
tr = {
        "A": 1,
        "B": 2,
        "C": 3,
        "X": 1,
        "Y": 2,
        "Z": 3,
}

def score(f,s):
    # adjust the sums since BZ would be a win for second 
    # it is instead treated as a loss since everything is mod 3
    # opposite for CY, therefore an adjustment is needed
    # that +- 1 in both cases would have effect only on these 2 sums
    adj = {"A" : 0,'B': -1,'C': 1}[f] 
    fs = (tr[f]+adj) % 3
    ss = (tr[s]+adj) % 3
    res =(tr[s] + (ss == fs) * 3 + (fs < ss)*6) 
    return res 

def score_s2(f,s):
    order = ['X','Y','Z']
    shftDir = {'A':-1,'B':0,'C':1}[f]
    order = order[shftDir:] + order[:shftDir]
    return score(f,order[ord(s) - ord('X')])

def test_score():
    res = {}
    print("  ",end = "")
    for j in 'XYZ':
        print(j,end=" ")
    print()
    for i in 'ABC':
        print(i,end=" ")
        for j in 'XYZ':
            print(score(i,j),end=" ")
        print()

def scenario1(fname):
    res = 0
    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            result = score(line[0],line[1])
            res += result
    print("scenario1",fname+ "\t",res)


def scenario2(fname):
    res = 0
    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            result = score_s2(line[0],line[1])
            res += result
    print("scenario2",fname+ "\t",res)

if __name__ == "__main__":
    test_score()
    scenario1("./input")
    scenario1("./input1")
    scenario2("./input")
    scenario2("./input1")
