package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)
func asBit(i int) int {
    return 1 << i
}

func decodeBits(i int,tar int) string {
    var res string
    var cnt int
    for ;i>0; i >>= 1 {
        if i & 1 == tar {
            res+=string(rune('A'+cnt))
        }
        cnt++
    }
    return res
}



func Part1(fname string) string {
    var f, err = os.Open(fname)
    var transition map[byte]int = make(map[byte]int)
    var middleNodes int

    if err != nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)

    for scanner.Scan() {
        var line = strings.Fields(scanner.Text())
        var from byte = line[1][0]
        var to byte = line[7][0]
        transition[from] |= asBit(int(to - 'A'))
        fmt.Println(from, to,transition[from])
        middleNodes |= asBit(int(to - 'A'))
    }
    for i := range transition {
        fmt.Printf("%c -> %s\n",i,decodeBits(transition[i],1))
    }
    // fmt.Printf("%c\n",'A' + emptyPos(middleNodes))

    fmt.Printf("%s\n",decodeBits(middleNodes,0))

    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return ""
}

func main() {
    fmt.Printf("Part 1 %s",Part1("input_test"))
}
