#!/usr/bin/python3
from Levenshtein import distance as lev

def substr_idx(string,pat):
    return [i for i in range(len(string)) if string.startswith(pat, i)]


def find_perm(fallout,mol):
    cache = {}
    for src in fallout:
        for res in fallout[src]:
            sl = len(src)
            rl = len(res)
            for i in substr_idx(mol,str(src)):
                result = mol[:i] + res + mol[i+sl:]
                if result not in cache:
                    cache[result] = 0
                cache[result] += 1
    return len(list(cache.keys()))


def sceanrio1():
    fallout = {}
    mol = ""
    with open("./input") as f:
        for line in f.readlines():
            l = line[:-1].split()
            if len(l) == 1:
                mol = l[0]
            elif len(l) == 0:
                pass
            else:
                if str(l[0]) not in fallout:
                    fallout[str(l[0])] = []
                fallout[str(l[0])].append(str(l[2]))
    print(find_perm(fallout,mol))


# i am shaken that this actually works
# keep shrinking the molecule to the one that has the most possibilities after
# my guess is that keeps combinations that have overlapping parts from leaving 
# unusable artifacts

def shrink(fallout, crnt_mol, target_mol):
    max_heur = -1
    max_str = ""
    for src in fallout:
        res = fallout[src]
        sl = len(src)
        for i in substr_idx(crnt_mol,str(src)):
            result = crnt_mol[:i] + res + crnt_mol[i+sl:]
            heur = find_perm(fallout,result)
            if max_heur == -1:
                max_heur = heur
                max_str = result
            max_heur,max_str = max((max_heur,max_str),(heur,result),key= lambda x: x[0])
    # print(min_str,min_lev)
    return max_str,max_heur


def transformBack(fallout,mol):
    target_str = "e"
    counter = 0
    # flip fusion to be a reverse translation table
    fusion = {}
    for comp in fallout:
        for res in fallout[comp]:
            fusion[res] = comp
    crnt_mol = mol

    while True:
        crnt_mol,dst = shrink(fusion,crnt_mol,target_str)
        if dst == -1:
            break
        counter += 1
    print(target_str,"in",counter)
    return counter



def sceanrio2():
    fallout = {}
    mol = ""
    with open("./input") as f:
        for line in f.readlines():
            l = line[:-1].split()
            if len(l) == 1:
                mol = l[0]
            elif len(l) == 0:
                pass
            else:
                if str(l[0]) not in fallout:
                    fallout[str(l[0])] = []
                fallout[str(l[0])].append(str(l[2]))
    transformBack(fallout,mol)


if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
