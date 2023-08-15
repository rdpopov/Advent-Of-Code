package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const HOUR = 0
const MIN = 1

type Node struct {
    ch rune
    nxt *Node
}

type List = struct {
    begin *Node
    end *Node
    length int
    crnt_node *Node
}

func append(l List)(ch rune) {
    var n = new(Node)
    n.ch = ch
    if l.begin == nil && l.end == nil {
        l.begin = n
        l.end = n
    } else {

    }
    return
}

func remove(l List)(ch rune) {
    return
}


func Part1(fname string) int {
    var f, err = os.Open(fname)

    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        var parsed = scanner.Text()
        fmt.Println(parsed)
    }

    if err := scanner.Err();err != nil {
        log.Fatal(err)
    }

    return 0
}

func main() {
    // jsut erm ... sort input
    fmt.Printf("Part 1 %d\n",Part1("input"))
    fmt.Printf("Part 1 %d\n",Part1("input_test"))

}
