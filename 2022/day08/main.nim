import sequtils
import bitops
import std/enumerate

const NORTH:int = 1
const SOUTH:int = 2
const EAST:int = 3
const WEST:int = 4

type
  Vec2 = tuple[x:int,y:int]

proc `+`(a,b:Vec2):Vec2 = 
  return (a.x + b.x, a.y + b.y)

proc between(a:Vec2,ux,uy:int):bool =
  return (0 <= a.x and a.x <= ux and 0 <= a.y and a.y <= uy)

proc iterateOverCollums(board:seq[seq[int]],mask:var seq[seq[int]],col:int) =
  var highest = -1
  for i in 0..board.high:
    if board[i][col] > highest:
      setbit(mask[i][col],NORTH)
      highest = board[i][col]
  highest= -1
  for i in countdown(board.high,0):
    if board[i][col] > highest:
      setbit(mask[i][col],SOUTH)
      highest = board[i][col]


proc iterateOverRows(board:seq[seq[int]],mask:var seq[seq[int]],row:int) =
  var highest = -1
  for i in 0..board[row].high:
    if board[row][i] > highest:
      setbit(mask[row][i],EAST)
      highest = board[row][i]
  highest = -1
  for i in countdown(board[row].high,0):
    if board[row][i] > highest:
      setbit(mask[row][i],WEST)
      highest = board[row][i]

proc scourWoods(board:seq[seq[int]],mask:var seq[seq[int]]):int=
  var visible:int = 0
  for col in 0..board.high:
    iterateOverCollums(board,mask,col)
  for row in 0..board[0].high:
    iterateOverRows(board,mask,row)
  for r in mask.items:
    for c in r.items:
      visible += int(c > 0)
  return visible

proc checkInDir(board:seq[seq[int]],pos:Vec2,d:Vec2): int =
  var crnt:Vec2 = pos
  var ux = board.high()
  var uy = board[0].high()
  crnt = crnt + d #mode it in thedirection of the vector
  while crnt.between(ux,uy):
    result += 1
    if board[crnt.x][crnt.y] >= board[pos.x][pos.y]:
      break
    crnt = crnt + d #mode it in thedirection of the vector
  return result



proc sonarWoods(board:seq[seq[int]]):int=
  for x in 1..<board.high:
    for y in 1..<board[0].high:
      var north = checkInDir(board,(x,y),(-1,0))
      var south = checkInDir(board,(x,y),(1,0))
      var east  = checkInDir(board,(x,y),(0,1))
      var west  = checkInDir(board,(x,y),(0,-1))
      result = max(result,north * south * east * west)

proc scenario1(fname:string): int = 
  var board:seq[seq[int]]
  var mask:seq[seq[int]]
  for l in lines(fname):
    board.add(l.toSeq().map(proc (x:char):int = x.int-'0'.int))
    mask.add(repeat(0,l.len).toSeq)
  return scourWoods(board,mask)

proc scenario2(fname:string): int = 
  var board:seq[seq[int]]
  var mask:seq[seq[int]]
  for l in lines(fname):
    board.add(l.toSeq().map(proc (x:char):int = x.int-'0'.int))
  return sonarWoods(board)


if isMainModule:
  echo scenario1("./input1")
  echo scenario1("./input")
  echo scenario2("./input1")
  echo scenario2("./input")
