import os
import parseutils
import sequtils

proc sign(x:int):int=
  return (if x < 1:0 else: 1)

proc increasing(readings:seq[int]):int=
  for i in 0..<readings.high:
    result = result + sign(readings[i+1] - readings[i])

proc Challange1(inp:seq[int])=
  echo increasing(inp)

proc Challange2(inp:seq[int])=
  var fmt_in:seq[int] = @[]
  for i in 0..(inp.high-2):
    fmt_in.add(inp[i]+inp[i+1]+inp[i+2])
  echo fmt_in.increasing

if isMainModule:
  var readings:seq[int] = @[]
  var tmp:int
  for line in lines "./input.txt":
    discard parseInt(line.string,tmp)
    readings.add(tmp)
  Challange1(readings)
