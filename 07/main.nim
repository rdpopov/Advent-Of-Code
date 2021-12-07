import math
import algorithm
import strutils
import sequtils

# not the most efficient but who cares nim is fast

proc formula(inp:seq[int],target:int):int64=
  return inp.mapIt(abs(it - target)).sum()

proc s(i,t:int):int=
  return abs(i-t) * (abs(i-t)+1) div 2

proc formula2(inp:seq[int],target:int):int64=
  return inp.mapIt(s(it,target)).sum()

proc Challange1(inp:seq[int],mi,mx,:int):int64=
  # could have gone with binary search .. .or some sort of jump search
  result = high(int64)
  for i in countup(mi,mx):
    result = min(result,formula(inp,i))


proc Challange2(inp:seq[int],mi,mx,:int):int64=
  # could have gone with binary search .. .or some sort of jump search
  result = high(int64)
  for i in countup(mi,mx):
    result = min(result,formula2(inp,i))

if isMainModule:
  var heights:seq[int]
  for line in lines "./input.txt":
    heights = line.split(',').mapIt(it.parseInt)
  heights.sort()
  var maxHeight:int = heights[heights.high]
  var minHeight:int = heights[0]

  echo Challange1(heights,minHeight,maxHeight)
  echo Challange2(heights,minHeight,maxHeight)
