import sequtils
import bitops

proc scenario1(fname:string): int = 
  var board:seq[seq[int]]
  for l in lines(fname):
    board.add(l.toSeq().map(proc (x:char):int = x.int))

proc scenario2(fname:string): int = 
    discard

if isMainModule:
  echo scenario1("./input1")
  echo scenario1("./input")
  echo scenario2("./input1")
  echo scenario2("./input")
