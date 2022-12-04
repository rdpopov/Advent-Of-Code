#!/usr/bin/python3
import re

def range_in_s1(f:list,s:list):
    if s[0]<=f[0] and f[1]<=s[1]:
        return 1
    if f[0]<=s[0] and s[1]<=f[1]:
        return 1
    return 0

def test_crit_s1():
    assert (range_in_s1([2,4],[1,3]) == 0), "partial overlap"
    assert (range_in_s1([1,2],[3,3]) == 0), "no overlap"
    assert (range_in_s1([2,3],[1,4]) == 1), "complete overlap"
    assert (range_in_s1([1,1],[1,1]) == 1), "same region"
    assert (range_in_s1([1,4],[1,3]) == 1), "lower bound equal"
    assert (range_in_s1([2,3],[1,3]) == 1), "upper bound equal"


def range_in_s2(f:list,s:list):
    if s[0]<=f[0]<=s[1]:
        return 1
    if f[0]<=s[0]<=f[1]:
        return 1
    return 0

def test_crit_s2():
    assert (range_in_s2([2,4],[1,3]) == 1), "partial overlap"
    assert (range_in_s2([1,2],[3,3]) == 0), "no overlap"
    assert (range_in_s2([2,3],[1,4]) == 1), "complete overlap"
    assert (range_in_s2([1,1],[1,1]) == 1), "same region"
    assert (range_in_s2([1,4],[1,3]) == 1), "lower bound equal"
    assert (range_in_s2([2,3],[1,3]) == 1), "upper bound equal"

def scenario1(fname):
    slackers = 0
    with open(fname) as f:
        for i in f.readlines():
            l = list(map(int,re.split(",|-",i[:-1])))
            # print(l, range_in(l[:2],l[2:]))
            slackers += range_in_s1(l[:2],l[2:])
    print(slackers)


def scenario2(fname):
    slackers = 0
    with open(fname) as f:
        for i in f.readlines():
            l = list(map(int,re.split(",|-",i[:-1])))
            # print(l, range_in(l[:2],l[2:]))
            slackers += range_in_s2(l[:2],l[2:])
    print(slackers)


if __name__ == "__main__":
    test_crit_s1()
    scenario1("./input")
    scenario1("./input1")
    test_crit_s2()
    scenario2("./input")
    scenario2("./input1")
