import math
import parseutils
import algorithm
import strutils
import sequtils

# not the most efficient but who cares nim is fast


proc Challange1(inp:seq[string]):int=
  for vol in 0..inp.high:
    for i in 0..inp[vol].high:
      block flt:
        if vol != 0:
          if inp[vol-1][i] <= inp[vol][i]: break flt
        if vol != inp.high:
          if inp[vol+1][i] <= inp[vol][i]: break flt
        if i != 0:
          if inp[vol][i-1] <= inp[vol][i]: break flt
        if i != inp[vol].high:
          if inp[vol][i+1] <= inp[vol][i]: break flt
        result = result + (inp[vol][i].int - '0'.int + 1)


if isMainModule:
  var heights:seq[string]
  for line in lines "./input.txt":
    heights.add line

  echo Challange1(heights)

