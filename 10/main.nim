import algorithm
import tables

let costs:Table[char,int]= {
  ')' : 3,
  ']' : 57,
  '}' : 1197,
  '>' : 25137,
  }.toTable

let lnk:Table[char,char]= {
  '(' : ')',
  '[' : ']',
  '{' : '}',
  '<' : '>',
  }.toTable

let opening:set[char] = {'(','{','[','<'}
let closing:set[char] = {')','}',']','>'}

proc is_valid(inp:string):int=
  var stk:seq[char]
  for i in inp:
    if i in opening:
      stk.add(i)
    elif i in closing:
      if stk.len != 0:
        if lnk[stk[stk.high]] == i:
          stk.delete(stk.high)
        else:
          return costs[i]
      else:
        return costs[i]
  return 0

proc Challange1(inp:seq[string]):int=
  for i in inp:
    result = result + is_valid(i)

let auto_costs:Table[char,int]= {
  '(' : 1,
  '[' : 2,
  '{' : 3,
  '<' : 4,
  }.toTable

proc complete(inp:string):uint64=
  var stk:seq[char]
  var res:uint64
  for i in inp:
    if i in opening:
      stk.add(i)
    elif i in closing:
      if stk.len != 0:
        if lnk[stk[stk.high]] == i:
          stk.delete(stk.high)
        else:
          return 0
      else:
        return 0
  for i in countdown(stk.high,0):
    res = res*5 + auto_costs[stk[i]].uint64
  return res

proc Challange2(inp:seq[string]):uint64=
  var res:seq[uint64]
  for i in inp:
    var tmp = complete(i)
    if tmp > 0:
      res.add(tmp)
  if res.len().bool:
    sort(res)
    return res[res.high div 2]
  return 0

if isMainModule:
  var inp:seq[string]
  for line in lines "./input.txt":
    inp.add(line)
  echo Challange1(inp)
  echo Challange2(inp)
