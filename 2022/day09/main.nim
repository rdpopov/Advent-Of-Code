import sequtils
import strutils
import sets
import bitops
import lists

type
  Vec2 = tuple[x:int,y:int]

proc dir2vec(dir:string):Vec2=
  case dir:
    of "R":
      return (0,1)
    of "L":
      return (0,-1)
    of "U":
      return (1,0)
    of "D":
      return (-1,0)
    else:
      return (0,0)

proc Dist(h,t:Vec2):int = 
  var dx = abs(t.x - h.x) 
  var dy = abs(t.y - h.y) 
  return max(dx,dy)

proc `+`(a,b:Vec2):Vec2 = 
  return (a.x + b.x, a.y + b.y)

proc Move(head,tail:var Vec2,mv:Vec2) = 
  var oldhead:Vec2 = head
  head = head + mv
  if Dist(head,tail) > 1:
    tail = oldhead

proc Move(head:var seq[Vec2],locations:var HashSet[Vec2] ,mv:Vec2) = 
  var i:int = 0
  var oldhead:Vec2 = head[i]
  head[i] = head[i] + mv
  head[i+1] = oldhead
  inc i
  while Dist(head[i],head[i+1]) > 1:
    oldhead = head[i]
    head[i] = head[i] + mv
    head[i+1] = oldhead
    locations.incl(head[9])
    inc i
    if i == head.len-1:
      break

proc display(rope:seq[Vec2],frSizeX,frSizeY:int) = 
  var dx:int = (frSizeX / 2).int
  var dy:int = (frSizeY / 2).int
  var frame:seq[seq[string]]= @[repeat('.',frSizeX)].repeat(frSizeY)
  frame[dy][0][dx] = 's'
  for i in countdown(rope.high,0):
    frame[rope[i].y + dy][0][rope[i].x + dx] = ($i)[0]
  for i in frame.items:
    echo i[0]
  echo ""


proc scenario1(fname:string): int = 
  var locations:HashSet[Vec2]
  var head:Vec2
  var tail:Vec2
  for l in lines(fname):
    var parsed= l.split()
    for i in 0..<parseInt(parsed[1]):
      Move(head,tail,dir2vec(parsed[0]))
      locations.incl(tail)
  return locations.len

proc scenario2(fname:string): int = 
  var locations:HashSet[Vec2]
  var rope:seq[Vec2] = repeat((0,0),10)
  for l in lines(fname):
    var parsed= l.split()
    for idx in 0..<parseInt(parsed[1]):
      locations.incl(rope[9])
      Move(rope,locations,dir2vec(parsed[0]))
      display(rope,20,10)
  return locations.len


if isMainModule: # echo scenario1("./input1")
  # echo scenario1("./input") # off by one?
  # echo scenario2("./input2")
  echo scenario2("./input1")
  # echo scenario2("./input")
