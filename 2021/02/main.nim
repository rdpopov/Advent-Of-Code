import os
import parseutils
import sequtils
import strutils

proc find_position1(inp:seq[(string,int)])=
  var down:int = 0
  var forward:int = 0
  for i in items(inp):
    case i[0]:
      of "up":
        down = down - i[1]
      of "forward":
        forward = forward + i[1]
      of "down":
        down = down + i[1]
  echo down * forward 

proc Challange1(inp:seq[(string, int)])=
  find_position1(inp)
  
proc find_position2(inp:seq[(string,int)])=
  var aim:int = 0
  var down:int = 0
  var forward:int = 0
  for i in items(inp):
    case i[0]:
      of "up":
        aim = aim - i[1]
      of "forward":
        forward = forward + i[1]
        down = down + (aim * i[1])
      of "down":
        aim = aim + i[1]
  echo down * forward 

proc Challange2(inp:seq[(string, int)])=
  find_position2(inp)

if isMainModule:
  var readings:seq[(string,int)] = @[]
  for line in lines "./input.txt":
    var tmp = (line.string).split()
    var tmp_int = 0
    discard parseInt(tmp[1],tmp_int)
    readings.add((tmp[0],tmp_int))
  Challange1(readings)
  Challange2(readings)
