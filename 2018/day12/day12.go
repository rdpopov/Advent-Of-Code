package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func parseState(state string) []int {
    var res []int = make([]int,len(state))
    for i,v:= range state {
        if v == '#' {
            res[i] = 1
        }
    }
    return res
}
func getState(arr []int,idx int ) int {
    if 0 <= idx && idx < len(arr) {
        return arr[idx]
    }
    return 0
}

func sumArr(arr []int) int {
    var res int
    for i,v := range arr {
        res += (i-2)*v
    }
    return res
}
func parseBits(state string) int {
    var res int
    for _,v:= range state {
        if v == '#' {
            res = res | 1
        }
        res <<= 1
    }
    return res
}

func parseTable(fname string) map[int]int {
    var f, err = os.Open(fname)
    var transition map[int]int = make(map[int]int)
    if err!= nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        var split = strings.Fields(scanner.Text())
        transition[parseBits(split[0])] = parseBits(split[2])
    }
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return transition
}

func Part1(initial_state string,fname string,iterations int) int{
    var transition = parseTable(fname)
    var crnt_arr = parseState(initial_state)
    var alt_arr = parseState(initial_state)
    for i:=0; i<iterations;i++ {
        for cell := range crnt_arr {
            var value = getState(crnt_arr,cell-2) << 4 | getState(crnt_arr,cell-1) << 3 | getState(crnt_arr,cell) << 2 | getState(crnt_arr,cell+1) << 1 | getState(crnt_arr,cell+2)  
            alt_arr[cell] = transition[value]
        }
        // for cell := range crnt_arr {
// crnt_arr[cell] = alt_arr[cell]
        // }
        crnt_arr , alt_arr = alt_arr, crnt_arr
    }
    return sumArr(crnt_arr)
}

func main() {
    //Part1("input_test")
    // fmt.Printf("Converged: %d\n", Part1("..###.#..#..##.##.###.#.....#.#.###.#.####....#.##..#.#.#..#....##..#.##...#.###.#.#..#..####.#.##.#..","input",20))
    fmt.Printf("Converged: %d\n", Part1("..#..#.#..##......###...###..","input_test",21))
    fmt.Printf("Converged: %d\n", Part1("..#..#.#..##......###...###..","input_test",20))
}
