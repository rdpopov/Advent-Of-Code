#!/usr/bin/python3.10

def prep_for_format(s):
    return s.count("\\") + s.count("\"") + len(s)

def count_actual(s):
    escaped = False
    count = 0
    for i in s:
        if i == "\\" and not escaped:
            escaped = True
        elif i != "x" and escaped:
            escaped = False
            count += 1
        elif i == "x" and escaped:
            escaped = False
            count -= 1
        else:
            count += 1
    return count

def sceanrio1():
    sum = 0
    with open("./input") as f:
        for i in f.readlines():
            line = i.strip()
            sum += len(line) - count_actual(line) + 2  # for the 2 characters around line
        print(sum)

def sceanrio2():
    sum = 0
    with open("./input") as f:
        for i in f.readlines():
            line = i.strip()
            sum += prep_for_format(line) - len(line) + 2 # for the 2 characters the string needs after its adjusted
        print(sum)

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
