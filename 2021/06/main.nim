import math
import strutils
import sequtils

proc roll[T](a:var openArray[T])=
  if len(a).bool:
    var fst:T = a[0]
    for i in 1..a.high:
      a[i-1] = a[i]
    a[a.high] = fst

proc Challange1(pop:var array[9,int],gen:int):int=
  for i in 0..(gen-1):
    pop.roll()
    pop[6] += pop[8]
  return pop.sum()

if isMainModule:
  var arr:array[9,int]
  for line in lines "./input.txt":
    var tmp = line.split(',').mapIt(it.parseInt)
    for i in items tmp:
      inc arr[i]

  echo Challange1(arr,80)
  echo Challange1(arr,256-80) # since challange 1 and 2 are the same just with more generations reuse the array
