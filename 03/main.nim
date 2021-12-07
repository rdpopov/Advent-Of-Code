import parseutils
import bitops

proc stat(readings:seq[int],size:int):int=
  var gamma:int
  var eta:int
  var mask:int
  var bits:array[64,int]
  for i in items(readings):
    for bts in 0..size:
      if i.testBit(bts):
        inc bits[bts] 
  for i in countdown(size-1,0):
    if (readings.len div 2 ) >  bits[i] :
      gamma.setBit(i)

  mask = toMask[int](0..<size)
  eta = gamma xor mask
  return eta * gamma


proc Challange1(inp:seq[int],size:int)=
  echo stat(inp,size)

proc filter_them(s:seq[int],test,bit_val:int):seq[int]=
  for i in s.items():
    if i.testBit(test).int == bit_val:
      result.add(i)
  return result
  

proc stat2(readings:seq[int],size:int):int=
  var res = readings
  var res2 = readings

  var count:int = 0
  for bt in countdown(size-1,0):
    count = 0
    for i in items(res):
      if i.testBit(bt):
        inc count

    echo count
    if res.len > 1:
      if count * 2 >= res.len:
        res = res.filter_them(bt,0)
      else:
        res = res.filter_them(bt,1)

    count = 0
    for i in items(res2):
      if i.testBit(bt):
        inc count
    echo count

    if res2.len > 1:
      if count * 2 >= res2.len:
        res2 = res2.filter_them(bt,1)
      else:
        res2 = res2.filter_them(bt,0)
  return res[0] * res2[0]

proc Challange2(inp:seq[int],size:int)=
   echo stat2(inp,size)


if isMainModule:
  var readings:seq[int] = @[]
  var tmp:int
  var ln:int
  for line in lines "./input.txt":
    discard parseBin("0b" & line.string,tmp)
    readings.add(tmp)
    if ln == 0:
      ln = line.string.len

  Challange1(readings,ln)
  Challange2(readings,ln)
