import strformat
import tables

proc parse(txt:string,len:int): int =
    var counts:Table[char,int]
    var sum:int = 0 
    for i in 'a'..'z':
        counts[i] = 0

    # prime table with values
    for i in 0..<len:
        sum += 1 shl counts[txt[i]]
        counts[txt[i]] += 1

    for idx in len..(txt.high-1):
        if sum == len:
            return idx
        counts[txt[idx-len]] -= 1
        sum -= 1 shl counts[txt[idx-len]]
        sum += 1 shl counts[txt[idx]]
        counts[txt[idx]] += 1

    return -1


proc scenario1(fname:string): int {.discardable.} =
    var f = open(fname)
    for l in f.lines():
        echo parse(l,4)

proc scenario2(fname:string): int {.discardable.} =
    var f = open(fname)
    for l in f.lines():
        echo parse(l,14)



if isMainModule:
    discard
    scenario1("./input1")
    scenario1("./input")
    scenario2("./input1")
    scenario2("./input")
