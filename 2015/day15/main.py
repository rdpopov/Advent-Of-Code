#!/usr/bin/python3
import json
from math import prod
def transpose(matr):
    return list(zip(*matr))

## big brain solution
def heur_s1(coef,matr,idx):
    d_coef = coef.copy()
    if idx is not None:
        d_coef[idx] += 1
    multiples = [sum([ci*vi for ci,vi in zip(d_coef,v)]) for v in matr]
    for i in multiples:
        if i  < 0:
            return 0
    return prod(multiples),idx

def heur_s2(coef,matr,idx):
    d_coef = coef.copy()
    if idx is not None:
        d_coef[idx] += 1

    multiples = [sum([ci*vi for ci,vi in zip(d_coef,v)]) for v in matr]
    for i in multiples:
        if i  < 0:
            return 0
    return prod(multiples[:-1]),multiples[-1],idx

# works very fast for case 1
def regress_s1(matr,length):
    matr_size = len(matr[0])
    coef = [4 for i in range(matr_size)]
    score = -1
    while sum(coef) != length:
        new_max,idx = max([heur_s1(coef,matr,idx)
                                for idx in range(matr_size)],
                          key=lambda x: x[0])
        if new_max > score:
            score = new_max
            coef[idx] += 1
    print(score,"with",coef)
    return(score,coef)

def regress_s2(matr,length):
    matr_size = len(matr[0])
    coef = [4 for i in range(matr_size)]
    score = -1
    while sum(coef) != length:
        new_max,energy,idx = max([heur_s2(coef,matr,idx)
                                for idx in range(matr_size)],
                          key=lambda x: x[0])
        if new_max > score:
            score = new_max
            coef[idx] += 1
    print(score,"with",coef)
    return(score,coef)

def best_cookie_intelligent(matr,length,heuristic):
    max_val = 0
    best_comb = []
    for i in gen_perm([],len(matr[0])):
        value = heuristic(i,matr)
        if value > max_val:
            max_val = value
            best_comb = i
    return (max_val,best_comb)

def sceanrio1():
    ingredients_name = []
    ingredients = []
    with open("./input") as f:
        for i in f.readlines():
            # line = i[:-1].split()
            line = i.split()
            name = line[0][:-1] # has : at end
            ingredients.append([])
            ingredients_name.append(name)
            for q in range(2,len(line),2):
                if ',' == line[q][-1]:
                    ingredients[-1].append(int(line[q][:-1]))

    ingredients = transpose(ingredients)
    regress_s1(ingredients,100)


# scenario 2 doesn't work that great, maybe with further tweaking
def sceanrio2():
    ingredients_name = []
    ingredients = []
    with open("./input") as f:
        for i in f.readlines():
            # line = i[:-1].split()
            line = i.split()
            name = line[0][:-1] # has : at end
            ingredients.append([])
            ingredients_name.append(name)
            for q in range(2,len(line),2):
                if ',' == line[q][-1]:
                    ingredients[-1].append(int(line[q][:-1]))
                else:
                    ingredients[-1].append(int(line[q]))

    ingredients = transpose(ingredients)
    regress_s2(ingredients,100)


if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
