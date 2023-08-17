package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)
func asBit(i uint) uint {
    return 1 << i
}

func decodeBits(i uint,tar uint) string {
    var res string
    var cnt int
    for ; i != 0; i >>= 1 {
        if i & 1 == tar {
            res+=string(rune('A'+cnt))
        }
        cnt++
        if cnt >  'Z' - 'A' {
            break
        }
    }
    return res
}



func Part1(fname string) string {
    var f, err = os.Open(fname)
    var transition map[byte]uint = make(map[byte]uint)
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
        fmt.Println(from, to,transition[from])
        middleNodes |= asBit(uint(to - 'A'))
    }
    for i := range transition {
        fmt.Printf("%c -> %s\n",i,decodeBits(transition[i],1))
    }
    // fmt.Printf("%c\n",'A' + emptyPos(middleNodes))
    var nodes_visited = (middleNodes) ^ ((1 << len(transition)+1) -1 )

    // redesign this!
    fmt.Printf("%b %s\n",nodes_visited,decodeBits(nodes_visited,0))
    var res []string = make([]string, 1)
    res[0] = decodeBits(nodes_visited,1)
    var lower_level_nodes uint
    for n:=2;n > 0; n-- {
        var nodes_visited_snapshot = nodes_visited
        for _,v := range decodeBits(nodes_visited ^ lower_level_nodes,1 ) {
            nodes_visited |= transition[byte(v)]
            fmt.Printf("%c ",v)
        }
        fmt.Printf("\n")
        lower_level_nodes = nodes_visited_snapshot
    }

    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return ""
}

func main() {
    fmt.Printf("Part 1 %s",Part1("input_test"))
    // fmt.Printf("Part 1 %s",Part1("input"))
}
