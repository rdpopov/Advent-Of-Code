import parseutils
import strutils
import sequtils
import bitops
import tables



proc get_collums(inp:seq[int]):seq[int]=
  result.add(inp[0..4])
  result.add(inp[5..9])
  result.add(inp[10..14])
  result.add(inp[15..19])
  result.add(inp[20..^25])
  result.add(inp[20..^25])
  for i in 0..4:
    var tmp:seq[int]
    for j in 0..4:
      tmp.add(inp[j*5+i])
    result.add(tmp)



if isMainModule:
  var call_order:seq[int] = @[]
  var delimited: seq[string] = @[""]
  var bingos:seq[seq[int]]
  var rev_index:Table[int,int]
  var iter:int =0
  var mode = 0

  for line in lines "./input.txt":
    if line.len == 0:
      inc iter
      delimited.add("")
    else:
      delimited[iter] = delimited[iter] & " " & line.string
  #call_order = delimited[0].split(',').mapIt(it.parseInt())
  call_order =  delimited[0].split({',',' '}).filterIt(it != "").mapIt(it.parseInt())
  bingos = delimited[1..delimited.high].mapIt(it.split().filterIt(it != "").mapIt(it.parseInt))

