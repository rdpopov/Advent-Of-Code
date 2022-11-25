#!/usr/bin/python3
import json
def at_sec(speed,time,rest,second):
    loop_distance = speed * time
    loop_time = time + rest
    loops = second // loop_time
    left = second % loop_time
    if left > time:
        left = time
    return loops * loop_distance + (left * speed)

def sceanrio1():
    deers = {}
    second = 2503
    maxdeer = 0
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-1].split()
            name = line[0]
            speed = int(line[3])
            time = int(line[6])
            rest = int(line[-2])
            deers[name] = at_sec(**{"speed":speed,"time":time,"rest":rest,"second":second})
            maxdeer = max(maxdeer,deers[name])
            # print(name,deers[name])
    # print(maxdeer)



def sceanrio2():
    deers = {}
    second = 2503
    # second = 10
    maxdeer = 0
    leaderboard = {}
    with open("./input") as f:
        for i in f.readlines():
            line = i[:-1].split()
            name = line[0]
            speed = int(line[3])
            time = int(line[6])
            rest = int(line[-2])
            deers[name] = {"speed":speed,"time":time,"rest":rest}
            leaderboard[name] = 0

    for i in range(1,second):
        snap = [(d,at_sec(second=i,**deers[d])) for d in deers]
        print(snap)
        distance = max( snap,key=lambda x:x[1])
        print(distance)
        for leadername,d in snap: 
            if distance[1] == d:
                leaderboard[leadername] +=1

    for i in leaderboard:
        print(i,leaderboard[i])



if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
