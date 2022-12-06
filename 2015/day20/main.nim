import math

proc sum_presents_1(N:int64): int64 =
    var lim:int = sqrt(N.float).int
    var sum:int64
    for i in 1..lim:
        if N mod i == 0:
            sum += N div i
            sum += i
    return sum * 10


proc scenario1(N:int64) : int64= 
    result = 0
    var num:int64 = 0
    while true: # i am a dumbass, should be a t least as many not equal ... learn to read damn you
        if sum_presents_1(num) > N:
            return num
        num += 1

proc sum_presents_2(N:int64): int64 =
    var lim:int = sqrt(N.float).int
    var sum:int64
    for i in 1..lim:
        if N mod i == 0:
            if i <= 50:
                sum += N div i
            if (N div i) <= 50:
                sum += i
    return sum * 11

proc scenario2(n:int64) : int64= 
    result = 0
    var num:int64 = 0
    while true: # i am a dumbass, should be a t least as many not equal ... learn to read damn you
        if sum_presents_2(num) > n:
            return num
        num += 1


if isMainModule:
    echo scenario1(36000000)
    echo scenario2(36000000)
