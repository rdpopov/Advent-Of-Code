#!/usr/bin/python3

def scenario1(fname):
    regX = 1
    cycle = 2
    snapshots = 0
    with open(fname) as f:
        for i in f.readlines():
            line = i.split()
            if line[0] == "addx":
                if (cycle - 20) % 40 == 0:
                    # print(1,regX,cycle)
                    snapshots += regX*cycle
                regX += int(line[1])
                cycle += 1
            if (cycle - 20) % 40 == 0:
                # print(2,regX,cycle)
                snapshots += regX*cycle
            cycle += 1

    print(snapshots)

def scenario2(fname):
    pass
    regX = 1
    cycle = 1
    snapshots = 0
    crt = [' ' for i in range(240)]
    conveyor = []
    with open(fname) as f:
        for i in f.readlines():
            line = i.split()
            cycle += 1
            print(cycle,regX,conveyor)
            if line[0] == "addx":
                conveyor.append((cycle+2,int(line[1])))
            if (cycle-1)%40 in [regX,regX-1,regX+1]:
                # print("draw @{cycle} {sprite}".format(cycle=cycle,sprite=regX))
                crt[cycle] = '#'
                # print(cycle,conveyor)
            print(cycle,regX,conveyor)
            if len(conveyor) > 0:
                if conveyor[0][0] == cycle:
                    regX += conveyor[0][1]
                    conveyor.pop(0)
            print(cycle,regX,conveyor)
    for i in range(6):
        print(''.join(crt[i*40:((i+1)*40)]))
        # print(crt[i*40:((i+1)*40)])
    pass


if __name__ == "__main__":
    # scenario1("./input1")
    # scenario1("./input")
    scenario2("./input1")
    # scenario1("./input1")
    # scenario1("./input")
    # scenario2("./input"w
