package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Worker struct{
    load rune
    time int
}

func asBit(i uint) uint {
    return 1 << i
}

func decodeBits(i uint,max_bits int) string {
    var res string
    var cnt int
    for ; cnt <= max_bits; i >>= 1 {
        if i & 1 == 1 {
            res+=string(rune('A'+cnt))
        }
        cnt++
        if cnt >  'Z' - 'A' {
            break
        }
    }
    return res
}
func free_worker_idx(arr []Worker) int {
    for i:= range arr {
        if arr[i].time == 0 {
            return i
        }
    }
    return -1
}



func Part1(fname string) string {
    var f, err = os.Open(fname)
    var transition map[byte]uint = make(map[byte]uint)
    var dependancy map[byte]uint = make(map[byte]uint)
    var middleNodes uint

    if err != nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)

    for scanner.Scan() {
        var line = strings.Fields(scanner.Text())
        var from byte = line[1][0]
        var to byte = line[7][0]
        transition[from] |= asBit(uint(to - 'A'))
        dependancy[to] |= asBit(uint(from - 'A'))
        middleNodes |= asBit(uint(to - 'A'))
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }
    var execution_path = ""
    var left uint = ( 1 << (len(transition)+1) )-1
    var done_nodes uint
    for ;; {
        var best = decodeBits(left,len(transition))
        if len(best) > 0 {
            var crnt_letter byte
            for _,v := range best  {
                if done_nodes | dependancy[byte(v)] == done_nodes {
                    crnt_letter = byte(v)
                    left ^= asBit(uint(crnt_letter- 'A'))
                    done_nodes |= asBit(uint(crnt_letter- 'A'))
                    // fmt.Printf("%s -> %c\n",execution_path,crnt_letter)
                    break
                }
            }
            execution_path += string(crnt_letter)
        } else {
            break
        }
    }
    return execution_path
}

func Part2(fname string,nworkers int,time_per_op int) (string,int) {
    var f, err = os.Open(fname)
    var transition map[byte]uint = make(map[byte]uint)
    var dependancy map[byte]uint = make(map[byte]uint)
    var middleNodes uint

    if err != nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)

    for scanner.Scan() {
        var line = strings.Fields(scanner.Text())
        var from byte = line[1][0]
        var to byte = line[7][0]
        transition[from] |= asBit(uint(to - 'A'))
        dependancy[to] |= asBit(uint(from - 'A'))
        middleNodes |= asBit(uint(to - 'A'))
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }
    var execution_path = ""
    var left uint = ( 1 << (len(transition)+1) )-1
    var done_nodes uint
    // var in_progress uint
    var workers []Worker = make([]Worker, nworkers+1)
    var done bool
    var moves int
    for ;; {
        var best = decodeBits(left,len(transition))
        if len(best) > 0 && !done {
            var crnt_letter byte
            for _,v := range best  {
                var worker_idx = free_worker_idx(workers)
                if worker_idx >= 0 {
                    if done_nodes | dependancy[byte(v)] == done_nodes {
                        crnt_letter = byte(v)
                        left ^= asBit(uint(crnt_letter- 'A'))
                        // in_progress |= asBit(uint(crnt_letter- 'A'))
                        workers[worker_idx].load = rune(crnt_letter)
                        workers[worker_idx].time = int(crnt_letter-'A'+1) + time_per_op
                        fmt.Printf("%s -> %c\n",execution_path,crnt_letter)
                    }
                }
            }
        } else {
            done = true
        }
        var zeroes int
        for i:= range workers {
            if workers[i].time==0 {
                zeroes ++
            } else {
                workers[i].time--
            }
            if workers[i].time == 0 && done_nodes | asBit(uint(workers[i].load- 'A')) != done_nodes {
                // in_progress ^= asBit(uint(workers[i].load- 'A'))
                done_nodes |= asBit(uint(workers[i].load- 'A'))
                execution_path += string(workers[i].load)
            }
        }
        if zeroes == len(workers) && done {
            break
        }
        moves ++
        // fmt.Println(workers)
    }
    return execution_path, moves
}



func main() {
    fmt.Printf("Part 1 %s\n",Part1("input_test"))
    fmt.Printf("Part 1 %s\n",Part1("input"))
    fmt.Printf("Part 2 ")
    fmt.Println(Part2("input_test",1,0))
    fmt.Printf("Part 2 ")
    fmt.Println(Part2("input",4,60))
}
