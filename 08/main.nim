import math
import algorithm
import strutils
import tables
import sets
import sequtils

# not the most efficient but who cares nim is fast
type
  code = object 
    signals:seq[string]
    todecode:seq[string]

proc srt(s:string):string=
   var p = s
   p.sort()
   return p

const needs:seq[HashSet[char]] = @[
  toHashSet(['a','b','c','e','f','g']),     # 0
  toHashSet(['c','f']),                     # 1
  toHashSet(['a','c','d','e','g']),         # 2
  toHashSet(['a','c','d','e','f','g']),     # 3
  toHashSet(['b','c','d','f']),             # 4
  toHashSet(['a','b','d','f','g']),         # 5
  toHashSet(['a','b','d','e','f','g']),     # 6
  toHashSet(['a','c','f']),                 # 7
  toHashSet(['a','b','c','d','e','f','g']), # 8
  toHashSet(['a','b','c','d','f','g']),     # 9
]


var desired_lengths:seq[int] = @[ needs[1].len, needs[4].len, needs[7].len, needs[8].len ]

proc Challange1(inp:seq[code]):int=
  for i in items inp:
    result = result + i.todecode.countIt(it.len in desired_lengths)

if isMainModule:
  var inp:seq[code]
  for line in lines "./input2.txt":
    var tmp = line.split('|')
    inp.add(code(signals:tmp[0].split().filterIt(it != ""),
                 todecode:tmp[1].split().filterIt(it != "") ))
  echo Challange1(inp)
