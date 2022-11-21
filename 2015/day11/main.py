#!/usr/bin/python3
def consecLetters(p):
    for c in zip(p[:-2],p[1:-1],p[2:]):
        if ord(c[1]) - ord(c[0]) == 1  and ord(c[2]) - ord(c[1]) == 1:
            return True
    return False

def noConfusion(p):
    return 'i' not in p and 'o'not in p and 'l' not in p

def twoTwins(p):
    cache = []
    pairs_used = []
    count = 0
    for i in p:
        cache.append(i)
        if len(cache) == 2:
            if cache[0] == cache[1] and cache[0] not in pairs_used:
                count+=1
                pairs_used.append(cache[0])
                cache = []
            else:
                cache = cache[1:]
    p_str = ''.join(p)
    for i in pairs_used:
        k = i * 3
        if k in p_str:
            return False
    return count > 1

def validatePass(p):
    return consecLetters(p) and noConfusion(p) and twoTwins(p)

def iteratePass(p):
    loc = list(p)
    idx = last =  len(p) - 1
    while (1):
        if loc[idx] == 'z':
            loc[idx] = 'a'
            idx -= 1
        else:
            loc[idx] = chr(ord(loc[idx]) + 1)
            if idx != last:
                idx = last
        if validatePass(loc):
            return ''.join(loc)


def sceanrio1():
    p = 'vzbxkghb'
    # p = 'ghijklmn'
    # p = 'abcdffaa'
    p = iteratePass(p)
    print(p)

def sceanrio2():
    p = 'vzbxxyzz'
    p = iteratePass(p)
    print(p)
if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
