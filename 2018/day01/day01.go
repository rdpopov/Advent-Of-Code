package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func Part1(fname string) int {
    var file, err = os.Open(fname)
    var ret int = 0
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    var scanner = bufio.NewScanner(file)
    for scanner.Scan() {
        var crnt int = 0
        _, err := fmt.Sscan(scanner.Text(),&crnt)
        if err != nil {
            log.Fatal(err)
        }
        ret += crnt
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return ret
}

func Part2(fname string) (int,error) {
    var file, err = os.Open(fname)
    var seen map[int]bool = make(map[int]bool)
    var res int = 0
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    var scanner = bufio.NewScanner(file)
    var items = make([]int,0)
    for scanner.Scan() {
        var crnt int = 0
        _, err := fmt.Sscan(scanner.Text(),&crnt)
        if err != nil {
            log.Fatal(err)
        }
        items = append(items,crnt)
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
        return 0, err
    }
    for idx:=0;; {
        res += items[idx]
        if seen[res] == true {
            return res, nil
        } else {
            seen[res] = true
        }
        idx += 1
        idx %= len(items)
    }
}

func main() {
    fmt.Println("Part 1",Part1("input"));
    var res, err = Part2("input");
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println("Part 2",res);
}
