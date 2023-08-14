package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)


type ByLev []string

func stupid_lev(this,other string) int{
    var res int
    if len(this) != len(other) {
        return 0
    }
    for i,_ := range this {
        if this[i] != other[i] {
            res += 1;
        }
    }
    return res
}


func remove_different(this,other string) string{
    var res = ""
    for i,_ := range this {
        if this[i] == other[i] {
            res += string(this[i])
        }
    }
    return res
}

func parse_hash_part1 (hsh string) map[int]int { // associative array
    var res = make(map[int]int,0)
    var count = make(map[rune]int)
    for _, ch := range hsh {
        count[ch]+=1
    }
    for _,v := range count {
        res[v]=1
    }
    return res
}

func Part1(fname string) int {
    var file, err = os.Open(fname)
    var res map[int]int =  make(map[int]int)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    var scanner = bufio.NewScanner(file)
    for scanner.Scan() {
        var hash_result = parse_hash_part1(scanner.Text())
        for k := range hash_result {
            res[k] += hash_result[k]
        }
    }
    return res[2] * res[3]
}

func Part2(fname string) string {
    var file, err = os.Open(fname)
    var hashes = make([]string,0)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    var scanner = bufio.NewScanner(file)
    for scanner.Scan() {
        hashes=append(hashes, scanner.Text())
    }
    for i:=0;i<(len(hashes)-1);i++{
        for j:=i+1;j<len(hashes);j++{
            if i!=j && stupid_lev(hashes[i],hashes[j]) == 1 {
                return remove_different(hashes[i],hashes[j])
            }
        }
    }
    return ""
}

func main() {
    fmt.Println("Part 1",Part1("input_test"));
    fmt.Println("Part 1",Part1("input"));
    fmt.Println("Part 2",Part2("input_test_2"));
    fmt.Println("Part 2",Part2("input"));
}
