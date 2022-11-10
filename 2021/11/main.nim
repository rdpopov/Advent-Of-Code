import algorithm
import parseutils
import strutils
import sequtils
import tables

var global:int = 0


proc update(inp:var seq[seq[int]],i,j:int)=
  if i < 0 or i> inp.high:
    return
  if j < 0 or j > inp[i].high:
    return

  if inp[i][j] == 0:
    return

  if inp[i][j] >= 9:
    inc global
    inp[i][j] = 0
    inp.update(i+1,j+1)
    inp.update(i+1,j-1)
    inp.update(i-1,j+1)
    inp.update(i-1,j-1)
    inp.update(i,j+1)
    inp.update(i,j-1)
    inp.update(i+1,j)
    inp.update(i-1,j)
    return

  inc inp[i][j]

  return

proc Challange1(inp:var seq[seq[int]])=
  # for i in 0..99:
  for i in 0..inp.high:
    for j in 0..inp[0].high:
      inc inp[i][j]

  for i in 0..inp.high:
    for j in 0..inp[0].high:
      if inp[i][j] > 9:
        update(inp,i,j)

proc Challange2(inp:var seq[seq[int]]):int=
  var pos:int = 0
  while true:
    inc pos
    for i in 0..inp.high:
      for j in 0..inp[0].high:
        inc inp[i][j]

    for i in 0..inp.high:
      for j in 0..inp[0].high:
        if inp[i][j] > 9:
          update(inp,i,j)

    if inp.mapIt(it.all(proc (x:int): bool = x == 0)).allIt(it == true):
      echo pos
      return pos


proc PrettyPrint(inp:seq[seq[int]])=
  for i in items inp:
    echo i.mapIt($it).join(" ")

if isMainModule:
  var inp:seq[seq[int]]
  for line in lines "./input.txt":
    inp.add line.mapIt(it).mapIt(parseInt($it))
  PrettyPrint(inp)

  # for i in 0..99:
  #   Challange1(inp)
  echo Challange2(inp)


