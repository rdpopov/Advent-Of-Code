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
      return (1,0)
    of "L":
      return (-1,0)
    of "U":
      return (0,-1)
    of "D":
      return (0,1)
    else:
      return (0,0)

proc Dist(h,t:Vec2):int = 
  var dx = abs(t.x - h.x) 
  var dy = abs(t.y - h.y)
  return max(dx,dy)

proc `+`(a,b:Vec2):Vec2 = 
  return (a.x + b.x, a.y + b.y)
proc `-`(a,b:Vec2):Vec2 = 
  return (a.x - b.x, a.y - b.y)

proc toDirection(a:Vec2):Vec2 = 
  var x:int = a.x
  var y:int = a.y
  if y != 0:
    y = y div abs(y)
  if x != 0:
    x = x div abs(x)
  return (x, y)

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

proc display_locs(rope:HashSet[Vec2],frSizeX,frSizeY:int) = 
  var dx:int = (frSizeX / 2).int
  var dy:int = (frSizeY / 2).int
  var frame:seq[seq[string]]= @[repeat('.',frSizeX)].repeat(frSizeY)
  frame[dy][0][dx] = 's'
  for x,y in rope.items:
    frame[y + dy][0][x + dx] = '#'
  for i in frame.items:
    echo i[0]
  echo ""

proc Move(head:var seq[Vec2],locations:var HashSet[Vec2] ,mv:Vec2) = 
  var i:int = 0
  var oldhead:seq[Vec2] = head.items.toSeq
  head[i] = head[i] + mv
  for i in 1..head.high:
    if Dist(head[i-1],head[i]) > 1:
      var dirvect:Vec2 = toDirection(head[i-1] - head[i])
      head[i] = head[i] + dirvect
    else:
      break
  locations.incl(head[head.high])


proc scenario1(fname:string): int = 
  var locations:HashSet[Vec2]
  # var head:Vec2
  # var tail:Vec2
  var rope:seq[Vec2] = repeat((0,0),2)
  for l in lines(fname):
    var parsed= l.split()
    for i in 0..<parseInt(parsed[1]):
      Move(rope,locations,dir2vec(parsed[0]))
  return locations.len

proc scenario2(fname:string): int = 
  var locations:HashSet[Vec2]
  var rope:seq[Vec2] = repeat((0,0),10)
  for l in lines(fname):
    var parsed= l.split()
    for idx in 0..<parseInt(parsed[1]):
      locations.incl(rope[9])
      Move(rope,locations,dir2vec(parsed[0]))
  return locations.len


if isMainModule:
  echo scenario1("./input")
  echo scenario2("./input2")
  echo scenario2("./input1")
  echo scenario2("./input")
