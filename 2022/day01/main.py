#!/usr/bin/python3
def sceanrio1():
    elf = 0
    max_cal = 0
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-1]
            if len(line) == 0:
                max_cal = max(max_cal, elf)
                elf = 0
            else:
                elf += int(line)
    print(max_cal)

def sceanrio2():
    elves = [0,0,0]
    elf = 0
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-1]
            if len(line) == 0:
                if elf > elves[-1]:
                    elves[-1] = elf
                elves.sort(reverse=True)
                elf = 0
            else:
                elf += int(line)
    print(sum(elves))

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
