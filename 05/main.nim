import sequtils
import algorithm
import strutils

type
  line = object
    x1:int
    y1:int
    x2:int
    y2:int

proc Custom(inp:string): line =
  var tmp = inp.split(',').mapIt(it.parseInt())
  result = line(x1:tmp[0],y1:tmp[1],x2:tmp[2],y2:tmp[3])

func valid_horiz(x:line):bool = 
  return x.x1 == x.x2 xor x.y1 == x.y2

func valid_diag(x:line):bool = 
  return ((x.x1 - x.x2) == (x.y1 - x.y2)) or ((x.x2 - x.x1) == (x.y2 - x.y1)) or (x.x1 + x.y1 == x.x2 + x.y2)

proc Challange1(inp:seq[line],mtr:var seq[seq[int]] ,maxX,maxY:int):int=
  for i in items inp:
    var xIter = if i.x1 > i.x2: i.x2..i.x1 else: i.x1..i.x2
    var yIter = if i.y1 > i.y2: i.y2..i.y1 else: i.y1..i.y2
    for x in xIter:
      for y in yIter:
        inc mtr[x][y]
  var acc:int
  for i in items mtr:
    inc(acc,i.countIt(it >= 2))
  return acc

proc Challange2(inp:seq[line],mtr:var seq[seq[int]] ,maxX,maxY:int):int=
  for i in items inp:
    var xIter = if i.x1 > i.x2: toSeq(i.x2..i.x1).reversed() else: toSeq(i.x1..i.x2)
    var yIter = if i.y1 > i.y2: toSeq(i.y2..i.y1).reversed() else: toSeq(i.y1..i.y2)
    for x,y in items zip(xIter,yIter):
      inc mtr[x][y]
  var acc:int
  for i in items mtr:
    inc(acc,i.countIt(it >= 2))
  return acc

if isMainModule:
  var horiz:seq[line] = @[]
  var diag:seq[line] = @[]
  var tmp:seq[seq[int]]
  var maxX,maxY:int
  for line in lines "./input.txt":
    var tmp = (line.string).split()
    var ln = Custom(tmp[0] & ',' & tmp[2])
    if valid_horiz ln:
      maxX=max([maxX,ln.x1,ln.x2])
      maxY=max([maxY,ln.y1,ln.y2])
      horiz.add(ln)
    if valid_diag ln:
      maxX=max([maxX,ln.x1,ln.x2])
      maxY=max([maxY,ln.y1,ln.y2])
      diag.add(ln)

  tmp.setLen(maxX+1)
  for i in mitems(tmp):
    i.setLen(maxY+1)
  echo Challange1(horiz,tmp,maxX,maxY)
  echo Challange2(diag,tmp,maxX,maxY)

