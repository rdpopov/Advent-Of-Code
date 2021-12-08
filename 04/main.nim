import parseutils
import math
import strutils
import sequtils
import bitops
import tables

var fin:(int,int) = (high(int),0)
proc last_idx(inp:seq[int],idx_table:Table[int,int])=
  var mx = inp[0]
  for i in items inp:
    if idx_table[mx] > idx_table[i]:
      mx = i
  if fin[0] > idx_table[mx]:
    fin = (idx_table[mx],inp.sum() * mx) 
    echo fin


proc get_collums(inp:seq[int],idx_table:Table[int,int])=
  var loc:seq[seq[int]]
  loc.add(inp[0..4])
  loc.add(inp[5..9])
  loc.add(inp[10..14])
  loc.add(inp[15..19])
  loc.add(inp[20..24])
  for i in 0..4:
    var tmp:seq[int]
    for j in 0..4:
      tmp.add(inp[j*5+i])
    loc.add(tmp)
  for i in items loc:
    last_idx(i,idx_table)

proc Challange1(inp:seq[seq[int]],idx_table:Table[int,int]):int=
  var res:seq[(int,int)]
  for i in items inp:
    echo i
    get_collums(i,idx_table)
  return fin[1]
    


if isMainModule:
  var call_order:seq[int] = @[]
  var delimited: seq[string] = @[""]
  var bingos:seq[seq[int]]
  var rev_index:Table[int,int]
  var iter:int =0
  var mode = 0

  for line in lines "./input2.txt":
    if line.len == 0:
      inc iter
      delimited.add("")
    else:
      delimited[iter] = delimited[iter] & " " & line.string
  #call_order = delimited[0].split(',').mapIt(it.parseInt())
  call_order =  delimited[0].split({',',' '}).filterIt(it != "").mapIt(it.parseInt())
  for i in 0..call_order.high:
    rev_index[call_order[i]] = i

  bingos = delimited[1..delimited.high].mapIt(it.split().filterIt(it != "").mapIt(it.parseInt))
  echo Challange1(bingos,rev_index)

