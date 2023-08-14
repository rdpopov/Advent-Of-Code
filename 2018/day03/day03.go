package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func Part1(fname string) int {
    var file, err = os.Open(fname)
    if err != nil {
        log.Fatal(err)
    }
    const board_size = 1500
    var board [][]int = make([][]int, board_size)
    for i := range board { 
        board[i]= make([]int,board_size)
    }

    defer file.Close()
    var scanner = bufio.NewScanner(file)
    for scanner.Scan() {
        var spl = strings.FieldsFunc(scanner.Text(),func(r rune) bool {return r== ' '|| r == '#'|| r == '@' || r == ',' || r == ':' || r == 'x'})
        var x,x_size int
        var y,y_size int
        fmt.Sscan(spl[1],&x)
        fmt.Sscan(spl[2],&y)
        fmt.Sscan(spl[3],&x_size)
        fmt.Sscan(spl[4],&y_size)
        for i:=0;i<x_size;i+=1{
            for j:=0;j<y_size;j+=1{
                board[i+x][j+y] += 1
            }
        }
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }

    var res int
    for i := range board { 
        for j := range board[i] { 
            if board[i][j] > 1 {
                res += 1
            }
        }
    }
    return res
}

func Part2(fname string) int {
    var file, err = os.Open(fname)
    if err != nil {
        log.Fatal(err)
    }
    const board_size = 1500
    var board [][]int = make([][]int, board_size)
    var visited map[int]bool = make(map[int]bool)
    for i := range board { 
        board[i]= make([]int,board_size)
    }

    defer file.Close()
    var scanner = bufio.NewScanner(file)
    for scanner.Scan() {
        var spl = strings.FieldsFunc(scanner.Text(),func(r rune) bool {return r== ' '|| r == '#'|| r == '@' || r == ',' || r == ':' || r == 'x'})
        var x,x_size int
        var y,y_size int
        var id int 
        fmt.Sscan(spl[0],&id)
        fmt.Sscan(spl[1],&x)
        fmt.Sscan(spl[2],&y)
        fmt.Sscan(spl[3],&x_size)
        fmt.Sscan(spl[4],&y_size)
        visited[id] = true
        for i:=0;i<x_size;i+=1{
            for j:=0;j<y_size;j+=1{
                if board[i+x][j+y] != 0 {
                    visited[board[i+x][j+y]] = false
                    visited[id] = false
                } else {
                    board[i+x][j+y] = id
                }
            }
        }
    }
    if err = scanner.Err(); err != nil {
        log.Fatal(err)
    }

    for i,is_overlap:= range visited {
        if  is_overlap {
            return i
        }

    }
    return 0
}

func main() {
    fmt.Println("Part 1",Part1("input_test"));
    fmt.Println("Part 1",Part1("input"));
    fmt.Println("Part 2",Part2("input_test"));
    fmt.Println("Part 2",Part2("input"));
}
