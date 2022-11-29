#!/usr/bin/python3
import json
from math import prod
def transpose(matr):
    return list(zip(*matr))

## unga bunga solution
def gen_perm(crnt,lvl):
    max_allowed = 100 - sum(crnt)
    i = 0
    while i <= max_allowed:
        if lvl > 0:
            yield from gen_perm(crnt + [i], lvl - 1)
        else:
            if sum(crnt) == 100:
                yield crnt
        i+=1

def heuristc_s1(coef,matr):
    multiples = [sum([ci*vi for ci,vi in zip(coef,v)]) for v in matr]
    # multiples = []
    # for v in matr:
    #     multiples.append(sum([ci*vi for ci,vi in zip(coef,v)]))
    for i in multiples:
        if i  < 0:
            return 0
    return prod(multiples)

def heuristc_s2(coef,matr):
    multiples = [sum([ci*vi for ci,vi in zip(coef,v)]) for v in matr]
    # multiples = []
    # for v in matr:
    #     multiples.append(sum([ci*vi for ci,vi in zip(coef,v)]))
    for i in multiples:
        if i  < 0:
            return 0
    if multiples[-1] == 500:
        return prod(multiples[:-1])
    else:
        return 0

def best_cookie(matr,length,heuristic):
    max_val = 0
    best_comb = []
    for i in gen_perm([],len(matr[0])):
        value = heuristic(i,matr)
        if value > max_val:
            max_val = value
            best_comb = i
    return (max_val,best_comb)

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
    print(best_cookie(ingredients,100,heuristc_s1))

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
    print(best_cookie(ingredients,100,heuristc_s2))

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
