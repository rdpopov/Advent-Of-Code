import algorithm
import math
import strutils
import strformat
import sequtils

proc make_board(inp:seq[(int,int)]):seq[string]=

  var mX,mY:int

  for y,x in items inp:
    mX = max(x,mX)
    mY = max(y,mY)
  result = sequtils.repeat(repeat('.',mY+1),mX+1)

  for y,x in items inp:
    result[x][y] = '#'

proc fold_board(inp:var seq[string],fold:(char,int))=

  var upper,lower:int = 0 
  var rev_move:bool=false

  if fold[0] == 'y':
    if fold[1] >= inp.len div 2:
      upper = inp.high
      lower = fold[1] * 2 - inp.high
    else:
      upper = fold[1] * 2
      lower = 0
      rev_move = true

    while upper != lower:
      inp[lower] = zip(inp[lower],inp[upper])
                .mapIt(if it[0] == '#' or it[1] == '#': '#' else: '.' )
                .join()
      inc lower
      dec upper 

    if rev_move:
      var tmp = inp[fold[1]*2 .. inp.high]
      tmp.reverse
      inp.setLen(fold[1])
      inp = concat(tmp,inp)
    else:
      inp.setLen(fold[1])

  if fold[0] == 'x':
    if fold[1] >= inp[0].len div 2:
      upper = inp[0].high
      lower = fold[1] * 2 - inp[0].high
    else:
      upper = fold[1] * 2
      lower = 0
      rev_move = true

    for i in 0..inp.high:
      var lHalf:string = inp[i][lower..<fold[1]]
      var uHalf:string = inp[i][fold[1]..upper]
      uHalf.reverse()
      # echo fmt"from {lower} to {fold[1]-1} : {lHalf}"
      # echo fmt"from {fold[1]} to {upper} : {uHalf}"
      var tmp:string

      if rev_move:
        tmp = inp[i][fold[1]*2 .. inp[i].high]
        tmp.reverse
      
      inp[i] = zip(lHalf,uHalf)
                .mapIt(if it[0] == '#' or it[1] == '#': '#' else: '.' )
                .join()
      if rev_move:
        inp[i] = tmp & inp[i]

    # for i in 0..inp.high:
    #   if rev_move:
    #     echo fold[1]*2 - 1
    #     echo inp[i].high
    #     var tmp = inp[i][fold[1]*2 .. inp[i].high]
    #     tmp.reverse
    #     inp[i].setLen(fold[1])
    #     inp[i] = tmp & inp[i]
    #   else:
    #     inp[i].setLen(fold[1])




proc Challange1(inp:var seq[(int,int)],folds:seq[(char,int)],times_fold:int):int=
  var board = make_board(inp)
  for i in 0..<times_fold:
    echo i
    fold_board(board,folds[i])
  # for i in items board:
  #   echo i
  return board.map(proc (x:string):int = x.countIt(it == '#')).sum()

proc Challange2(inp:var seq[(int,int)],folds:seq[(char,int)])=
  var board = make_board(inp)
  for i in items folds:
    echo i
    fold_board(board,i)
  for i in items board:
    echo i.split()


if isMainModule:
  var inp:seq[(int,int)]
  var folds:seq[(char,int)]
  for line in lines "./input.txt":
    if ',' in line:
      var tmp = line.split(',').mapIt(parseInt(it))
      inp.add((tmp[0],tmp[1]))
    if '=' in line:
      var tmp = line.split()[2].split('=')
      folds.add((tmp[0][0],tmp[1].parseInt))
  for i in folds:
    echo i 

  echo Challange1(inp,folds,folds.len)
  Challange2(inp,folds)
