import algorithm
import parseutils
import strutils
import strformat
import sets
import lists
import sequtils
import tables

proc Challange1(initial:string,react:Table[string,string],steps:int):int=
  var loc = initial
  # echo initial
  # for k,v in pairs react:
  #   echo fmt"{k} : {v}"
  for i in 0..<steps:
    echo i
    loc = zip(loc[0..<loc.high],loc[1..loc.high]).mapIt(it[0] & react[it[0] & it[1]]).join() & loc[loc.high]
    # echo fmt"step {i} : {loc}"
  var tbl = loc.toCountTable()
  return tbl.largest[1] - tbl.smallest[1]

proc Helper(initial:string,react:Table[string,string],steps:int):CountTable[char]=
  var loc = initial
  var lst:SinglyLinkedList[char]
  for i in loc:
    result.inc(i)
    lst.append(i)
  for i in 0..<steps:
    var tmp:SinglyLinkedNode[char] = lst.head
    var next:SinglyLinkedNode[char]
    while tmp.next != nil:
      next = tmp.next # temporary save it
      tmp.next = newSinglyLinkedNode[char](react[tmp.value & next.value][0]) # init the next member
      result.inc(tmp.next.value)
      (tmp.next).next = next # relink the list
      tmp = next # get to next member
  result[initial[initial.high]] = result[initial[initial.high]] - 1  # since the last element is the first in the next have to adjust it 

proc Helper(initial:string,after_iter:Table[string,CountTable[char]],react:Table[string,string],steps:int):CountTable[char]=
  var loc = initial
  var lst:SinglyLinkedList[char]
  for i in loc:
    result.inc(i)
    lst.append(i)
  for i in 0..<steps:
    var tmp:SinglyLinkedNode[char] = lst.head
    var next:SinglyLinkedNode[char]
    while tmp.next != nil:
      next = tmp.next # temporary save it
      tmp.next = newSinglyLinkedNode[char](react[tmp.value & next.value][0]) # init the next member
      (tmp.next).next = next # relink the list
      tmp = next # get to next member

  var tmp:SinglyLinkedNode[char] = lst.head
  while tmp.next != nil:
    result.merge(after_iter[tmp.value & tmp.next.value])
    tmp = tmp.next
  result[initial[initial.high]] = result[initial[initial.high]] - 1  # since the last element is the first in the next have to adjust it 

proc GenerateForNSteps(react:Table[string,string],steps:int):Table[string,CountTable[char]]=
  for i in react.keys:
    result[i] = Helper(i,react,steps)


proc Challange2(initial:string,react:Table[string,string],steps:int):int=
  var res:seq[string] = zip(initial[0..<initial.high],initial[1..initial.high]).mapIt(it[0] & it[1])
  var steps_to_cache:int =steps div 2
  var cache20 = GenerateForNSteps(react,steps_to_cache)
  echo "created chache"
  echo "created chache"
  var cache:Table[string,CountTable[char]]
  var final:CountTable[char]
  for i in items res:
    if not cache.hasKey(i):
      echo i
      cache[i] = Helper(i,cache20,react,steps - steps_to_cache)
  for i in items res:
    final.merge(cache[i])
  final.inc(initial[initial.high],1) # have to compensate for there missing 2 of the last character
  return final.largest[1] - final.smallest[1]

if isMainModule:
  var inp:Table[string,string]
  var initial:string
  for line in lines "./input.txt":
    if '-' in line:
      var tmp = line.split({'-','>',' '}).filterIt(it != "")
      inp[tmp[0]] = tmp[1]
    elif line.len != 0: 
      initial = line
  # echo Challange1(initial,inp,10)
  echo Challange2(initial,inp,40)
