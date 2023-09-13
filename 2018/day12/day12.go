package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)


func sumArr(arr []byte, offs int) int {
    var res int
    for i,v := range arr {
        if v == '#' {
            res += (i + offs)
        }
    }
    return res
}


func parseTable(fname string) map[string]byte {
    var f, err = os.Open(fname)
    var transition = make(map[string] byte)
    if err!= nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        var split = strings.Fields(scanner.Text())
        // fmt.Println(split[0],parseBits(split[0]),split[2],parseBits(split[2]))
        transition[split[0]] = split[2][0]
    }
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return transition
}

func getState(st []byte,cell int) string {
    if cell < 0 || len(st) <= cell {
        return string('.')
    }
    return string(st[cell])
}


func Part1(initial_state string,fname string,iterations int) int{
    var transition = parseTable(fname)
    var crnt_arr = []byte(strings.Repeat(".",30) + initial_state + strings.Repeat(".",iterations+100))
    var alt_arr = []byte(strings.Repeat(".",30) + initial_state + strings.Repeat(".",iterations+100))
    var offset int = -30

    for i:=0; i<iterations;i++ {
        for cell := range crnt_arr {
            var mapkey = getState(crnt_arr,cell -2) + getState(crnt_arr,cell -1) + getState(crnt_arr,cell + 0) + getState(crnt_arr,cell + 1) + getState(crnt_arr,cell + 2)
            if transition[mapkey] == 0 {
                alt_arr[cell] = '.'
            } else {
                alt_arr[cell] = transition[mapkey]
            }
        }
        crnt_arr,alt_arr = alt_arr,crnt_arr
    }
    return sumArr(crnt_arr,offset)
}

func Part2(initial_state string,fname string,iterations int,short_iterations int) int{
    var transition = parseTable(fname)
    var crnt_arr = []byte(strings.Repeat(".",300) + initial_state + strings.Repeat(".",3000))
    var alt_arr = []byte(strings.Repeat(".",300) + initial_state + strings.Repeat(".",3000))
    var offset int = -300
    var base_value []int = make([]int,short_iterations+1)
    for i:=0; i<short_iterations+1;i++ {
        base_value[i] = sumArr(crnt_arr,offset)
        for cell := range crnt_arr {
            var mapkey = getState(crnt_arr,cell -2) + getState(crnt_arr,cell -1) + getState(crnt_arr,cell + 0) + getState(crnt_arr,cell + 1) + getState(crnt_arr,cell + 2)
            if transition[mapkey] == 0 {
                alt_arr[cell] = '.'
            } else {
                alt_arr[cell] = transition[mapkey]
            }
        }
        copy(crnt_arr,alt_arr)
    }
    var diff = base_value[short_iterations] - base_value[short_iterations-1]

    return base_value[short_iterations] + (iterations-short_iterations) * diff
}

func main() {
    fmt.Printf("Converged: %d\n", Part1("###.#..#..##.##.###.#.....#.#.###.#.####....#.##..#.#.#..#....##..#.##...#.###.#.#..#..####.#.##.#","input",20))
    fmt.Printf("Converged: %d\n", Part2("###.#..#..##.##.###.#.....#.#.###.#.####....#.##..#.#.#..#....##..#.##...#.###.#.#..#..####.#.##.#","input",50000000000,2000))
}
