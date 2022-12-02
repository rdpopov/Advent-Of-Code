#!/usr/bin/python3
def substr_idx(string,pat):
    return [i for i in range(len(string)) if string.startswith(pat, i)]

def find_perm(fallout,mol):
    # print(*fallout)
    # print(mol)
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


def heuristic(possible,mol):
    # some heuristic based in levenstein distance
    # and keep a list of possible substitutions and their posiotions ... maybe or references
    # 
    pass

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



def sceanrio2():
    pass




if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
