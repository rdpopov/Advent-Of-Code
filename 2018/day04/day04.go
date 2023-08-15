package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

const HOUR = 0
const MIN = 1

type Guard struct {
    Guard_id int
    tbl_slept []int
    total_slept int
    shifts int
    slept_times int
}

func parse_flt(c rune) bool{
    return strings.Contains("[]-: #",string(c))
}

func has[V int | string](container []V, elem V ) bool {
    for _,v := range container {
        if elem == v {
            return true
        }
    }
    return false
}
func sum[V int | string](cont []V) V {
    var res V
    for _,v := range cont {
        res += v
    }
    return res
}

func max_value[V int | string](cont []V) V {
    var res int
    for i := range cont {
        if cont[res] <= cont[i] {
            res = i
        }
    }
    return cont[res]
}
func max_index[V int | string](cont []V) int {
    var res int
    for i := range cont {
        if cont[res] < cont[i] {
            res = i
        }
    }
    return res
}

func Part1(fname string, criteria func([]int)int) int{
    var f, err = os.Open(fname)
    var crnt_guard_id int
    var start_sleep int
    var end_sleep int
    var time_table = make(map[int]*Guard)
    var sleeping_guard int

    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        var parsed = strings.FieldsFunc(scanner.Text(),parse_flt)
        if has(parsed,"Guard") {
             fmt.Sscan(parsed[6], &crnt_guard_id)
             if time_table[crnt_guard_id] == nil {
                time_table[crnt_guard_id] = new(Guard)
                time_table[crnt_guard_id].tbl_slept = make([]int, 60)
                time_table[crnt_guard_id].Guard_id = crnt_guard_id 
             }
            time_table[crnt_guard_id].shifts +=1
        } else if has(parsed,"falls"){
            time_table[crnt_guard_id].slept_times +=1
            sleeping_guard = crnt_guard_id
            fmt.Sscan(parsed[4],&start_sleep)
        } else if has(parsed,"wakes"){
            fmt.Sscan(parsed[4],&end_sleep)
                for m:= start_sleep; m < end_sleep;m+=1 {
                    time_table[sleeping_guard].total_slept += 1
                    time_table[sleeping_guard].tbl_slept[m] += 1
                }
        }
    }
    var max_Guard *Guard
    var max_slept int
    for _,v:= range time_table {
        var slept int = criteria(v.tbl_slept)
        if max_slept < slept {
            max_Guard = v
            max_slept = slept
        }
    }

    if err := scanner.Err();err != nil {
        log.Fatal(err)
    }

    return max_Guard.Guard_id * max_index(max_Guard.tbl_slept)
}

func main() {
    // jsut erm ... sort input
    fmt.Printf("Part 1 %d\n",Part1("input_sorted",sum[int]))
    fmt.Printf("Part 1 %d\n",Part1("input_test",sum[int]))
    // Part 2
    fmt.Printf("Part 2 %d\n",Part1("input_sorted",max_value[int]))
    fmt.Printf("Part 2 %d\n",Part1("input_test",max_value[int]))

}
