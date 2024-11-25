package main
 
import (
	"fmt"
)
type Seq struct {
    line []int8
    crnt_size int
    max int
}

func Part1(maxlen int) []int8 {
    var seq Seq
    var elf1 = 0
    var elf2 = 1
    seq.max = maxlen + 20 
    seq.line = make([]int8, seq.max)
    seq.line[0] = 3
    seq.line[1] = 7
    seq.crnt_size = 2
    for seq.crnt_size < maxlen+15 {
        var score = seq.line[elf1] + seq.line[elf2]
        if score >= 10 {
            seq.line[seq.crnt_size] = 1
            seq.crnt_size ++
        }
        seq.line[seq.crnt_size] = score % 10
        seq.crnt_size ++
        elf1 = (elf1 + int(seq.line[elf1]) + 1) % seq.crnt_size
        elf2 = (elf2 + int(seq.line[elf2]) + 1) % seq.crnt_size
    }
    return seq.line[maxlen:maxlen+10]
}
func to_slice(repr string) []int8{
    var ret []int8 = make([]int8,len(repr))
    for i,d := range repr {
        ret[i] = int8(d - '0')
    }
    return ret
}
func check_for_pattern(seq Seq,didgits []int8 ) int {
    if seq.crnt_size > len(didgits) {
        var window = seq.line[seq.crnt_size-len(didgits)-1:seq.crnt_size-1]
        var correct = len(didgits)
        for i := range didgits {
            if didgits[i] == window[i] {
                correct --
            }
        }
        if correct == 0 {
            return seq.crnt_size-len(didgits)-1
        }
    }
    return -1
}

func Part2(pattern string ,alloc int) int {
    var seq Seq
    var elf1 = 0
    var elf2 = 1
    var didgits = to_slice(pattern)
    seq.max = alloc + 20 
    seq.line = make([]int8, seq.max)
    seq.line[0] = 3
    seq.line[1] = 7
    seq.crnt_size = 2
    for seq.crnt_size < alloc+15 {
        var score = seq.line[elf1] + seq.line[elf2]
        if score >= 10 {
            seq.line[seq.crnt_size] = 1
            seq.crnt_size ++
            var check = check_for_pattern(seq,didgits)
            if check > -1 {
                return check
            }
        }
        seq.line[seq.crnt_size] = score % 10
        seq.crnt_size ++

        var check = check_for_pattern(seq,didgits)
        if check > -1 {
            return check
        }
        elf1 = (elf1 + int(seq.line[elf1]) + 1) % seq.crnt_size
        elf2 = (elf2 + int(seq.line[elf2]) + 1) % seq.crnt_size
    }
    return -1
}

func main() {
    fmt.Printf("Score test Part1: %v \n", Part1(9))
    fmt.Printf("Score real Part1: %v \n", Part1(704321))
    fmt.Printf("Score test Part2: %v \n", Part2("51589",9))
    // no real need to optimize this, for what it does it's fast, might as well full send it
    fmt.Printf("Score real Part2: %v \n", Part2("704321",704321*100))
}
