#!/usr/bin/python3

priority_table = {}
for i in range(1,27):
    c = chr(ord('a') + i - 1)
    cu = chr(ord('A') + i - 1)
    priority_table[c] = i
    priority_table[cu] = i + 26

# print(str(priority_table))

def scenario1(fname):
    sum = 0
    with open(fname) as f:
        for i in f.readlines():
            l = i[:-1]
            left = l[len(l)//2:]
            rght = l[:len(l)//2]
            for c in left:
                if c in rght:
                    sum += priority_table[c]
                    break
    print("scenario1",fname+ "\t",sum)


def scenario2(fname):
    sum = 0
    cache = []
    with open(fname) as f:
        for i in f.readlines():
            l = i[:-1]
            cache.append(l)
            if len(cache) == 3:
                for char in cache[0]:
                    if char in cache[1] and char in cache[2]:
                        sum += priority_table[char]
                        break
                cache = []

    print("scenario2",fname+ "\t",sum)

if __name__ == "__main__":
    # test_score()
    scenario1("./input")
    scenario1("./input1")
    scenario2("./input")
    scenario2("./input1")
