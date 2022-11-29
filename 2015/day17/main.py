#!/usr/bin/python3
## unga bunga
def gen_perm_s1(pool,comb,target):
    if target == 0:
        yield comb
    if len(pool) > 0:
        if pool[0] <= target:
            yield from gen_perm_s1(pool[1:],comb + [pool[0]],target - pool[0])
            yield from gen_perm_s1(pool[1:],comb,target)

def gen_len_s2(pool,comb,target):
    if target == 0:
        yield len(comb)
    if len(pool) > 0:
        if pool[0] <= target:
            yield from gen_len_s2(pool[1:],comb + [pool[0]],target - pool[0])
            yield from gen_len_s2(pool[1:],comb,target)


def sceanrio1():
    containers = []
    perms = 0
    with open("./input") as f:
        for l in f.readlines():
            containers.append(int(l.strip()))
    containers.sort()
    for i in gen_perm_s1(containers,[],150):
        # print(i)
        perms += 1
    print(perms)

def sceanrio2():
    containers = {}
    lin = []
    perms = 0
    sum_size = 150
    with open("./input") as f:
        for l in f.readlines():
            num = int(l.strip())
            lin.append(num)
            if num in containers:
                containers[num] += 1
            else:
                containers[num] = 1

    set = list(containers.keys())
    set.sort()
    lin.sort()

    cache = {}

    for i in gen_len_s2(lin,[],sum_size):
        if i not in cache:
            cache[i] = 0
        cache[i] +=1
    print(str(cache))


    # print(perms)
    # for i in containers:
    #     if containers[i]>1:
    #         set.remove(i)
    #         for j in range(2,containers[i]+1):
    #             print(i)
    #             for k in gen_perm(set,[i] * j ,sum_size-i*j):
    #                 # print(k)
    #         set.append(i)
    #         set.sort()
    print(perms)



if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
