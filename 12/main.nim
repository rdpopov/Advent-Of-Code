import algorithm
import parseutils
import strutils
import strformat
import sets
import sequtils
import tables

proc special_in(name:string):bool=
  if name[0] in {'A'..'Z'}:
    return true
  return false

proc included_twice(path:seq[string]):bool=
  var tmp = toCountTable(path.filterIt(not special_in(it))).largest
  if  2 == tmp[1]:
    return false
  return true

proc DFS(cnode:string,path:var seq[string],inp:Table[string,seq[string]],paths:var seq[string])=
  path.add(cnode)
  if cnode == "end":
    paths.add(path.join(","))
    path.del(path.high)
    return
  for i in items inp[cnode]:
    if not (i in path):
      DFS(i,path,inp,paths)
    elif special_in(i):
      DFS(i,path,inp,paths)

  path.del(path.high)
  return


proc MutantDFS(cnode:string,path:var seq[string],inp:Table[string,seq[string]],paths:var seq[string])=
  path.add(cnode)
  if cnode == "end":
    paths.add(path.join(","))
    path.del(path.high)
    return
  for i in items inp[cnode]:
    if not (i in path):
      MutantDFS(i,path,inp,paths)
    elif special_in(i):
      MutantDFS(i,path,inp,paths)
    elif included_twice(path) and not (i ==  "start"):
      MutantDFS(i,path,inp,paths)
  path.del(path.high)
  return

proc Challange1(inp:Table[string,seq[string]])=
  var paths: seq[string] = @[]
  var path:seq[string] = @[]
  DFS("start",path,inp,paths)
  echo paths.len

proc Challange2(inp:Table[string,seq[string]])=
  var paths: seq[string] = @[]
  var path:seq[string] = @[]
  MutantDFS("start",path,inp,paths)
  echo paths.len


if isMainModule:
  var inp:Table[string,seq[string]]
  for line in lines "./input.txt":
    var tmp = line.split({'-'})
    if not inp.hasKey(tmp[0]):
      inp[tmp[0]] = @[]

    if not inp.hasKey(tmp[1]):
      inp[tmp[1]] = @[]

    inp[tmp[0]].add(tmp[1])
    inp[tmp[1]].add(tmp[0])

  for i in keys inp:
    echo fmt"{i} : {inp[i]}"
  Challange1(inp)
  Challange2(inp)
