package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func toupper(c byte)byte{
    if isLower(c) {
       return (c - 'a') + 'A'
    }
    return c
}

func isUpper(c byte) bool {
    return 'A' <= c && c <= 'Z'
}

func isLower(c byte) bool {
    return 'a' <= c && c <= 'z'
}

func sameLetter(f,s byte) bool {
    return f != s && strings.ToUpper(string(f)) == strings.ToUpper(string(s))
}

func upperAndLower(f,s byte) bool {
    return isUpper(f) && isLower(s) && sameLetter(f,s) ||
    isUpper(s) && isLower(f) && sameLetter(f,s)
}

func fold_pairs(parsed string,mask []bool) {
    var fst  = 0
    var sec  = 1
    for ;sec < len(mask); {

        if !mask[fst] {
            fst += 1
            if fst == len(mask) {
                return
            }
        }
        if !mask[sec] || sec == fst {
            sec += 1
            if sec == len(mask) {
                return
            }
        }
        if mask[fst] && mask[sec] && fst < sec {
            if upperAndLower(parsed[fst],parsed[sec]) {
                mask[fst] = false
                mask[sec] = false
                fst  = 0
                sec  = 1
            } else {
                fst ++
                sec ++
            }
        }
    }
}
func print(parsed string,mask []bool) {
    var cnt = 0
    for  i := range parsed{
        fmt.Printf("%c - %t\n",parsed[i],mask[i])
        if mask[i] {
            cnt++
        }
    }
    fmt.Printf("%d\n",cnt)
}
func len_of_string(mask []bool) int {
    var cnt = 0
    for v := range mask {
        if mask[v] {
            cnt++
        }
    }
    return cnt
}
func uniqueLetters(parsed string) map[byte]bool {
    var res = make(map[byte]bool)
    for _,v := range parsed {
        res[toupper(byte(v))] = true
    }
    return res
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
        var mask = make([]bool,len(parsed))
        for v := range mask{
            mask[v] = true
        }
        fold_pairs(parsed,mask)
        return len_of_string(mask)
    }
    if err := scanner.Err();err != nil {
        log.Fatal(err)
    }

    return 0
}

func Part2(fname string) int {
    var f, err = os.Open(fname)
    var res = make(map[byte]int)
    var min_len int 

    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        var parsed = scanner.Text()
        var mask = make([]bool,len(parsed))
        for letter := range uniqueLetters(parsed) {
            for v := range mask{
                mask[v] = true
                if toupper(parsed[v]) == letter {
                    mask[v] = false
                }
            }
            fold_pairs(parsed,mask)
            res[letter] = len_of_string(mask)
            min_len = res[letter]
        }
    }

    for _,v := range res {
        min_len = min(min_len,v)
    }
    if err := scanner.Err();err != nil {
        log.Fatal(err)
    }
    return min_len
}

func main() {
    fmt.Printf("Part 1 %d\n",Part1("input"))
    fmt.Printf("Part 1 %d\n",Part1("input_test"))
    fmt.Printf("Part 2 %d\n",Part2("input"))
    fmt.Printf("Part 2 %d\n",Part2("input_test"))

}
