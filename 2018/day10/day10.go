package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Point struct {
    x int
    y int
    dx int
    dy int
}

func (self *Point) Move () {
    self.x += self.dx
    self.y += self.dy
}

func (self *Point) FromOrigin () int {
    return self.x * self.x + self.y * self.y
}

func ParsePoints(fname string) []Point {
    var f, err = os.Open(fname)
    var points []Point = make([]Point, 0)
    if err!= nil {
        log.Fatal(err)
    }
    var scanner = bufio.NewScanner(f)
    for scanner.Scan(){
        // fmt.Println(scanner.Text())
        var split = strings.FieldsFunc(scanner.Text(), func(r rune) bool {
                return r == '<' || r == '>' || r == ','
        })
        var newPoint Point
        fmt.Sscan(split[1],&newPoint.x)
        fmt.Sscan(split[2],&newPoint.y)
        fmt.Sscan(split[4],&newPoint.dx)
        fmt.Sscan(split[5],&newPoint.dy)

        points = append(points, newPoint)

    }
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
    return points
}

func MovePoints(points []Point) {
    for i:= range points {
        points[i].Move()
    }
}

func Spread(points []Point) int {
    var min_node =  points[0].FromOrigin()
    var max_node =  points[1].FromOrigin()
    for i := range points {
        var vDst = points[i].FromOrigin()
        if min_node >= vDst {
            min_node = vDst
        }
        if max_node < vDst {
            max_node = vDst
        }
    }
    return max_node - min_node
}
func Print(points []Point) {
    var min_x int = points[0].x
    var max_x int = points[0].x
    var min_y int = points[0].y
    var max_y int = points[0].y

    for i := range points {
        min_x = min(points[i].x, min_x)
        min_y = min(points[i].y, min_y)
        max_x = max(points[i].x, max_x)
        max_y = max(points[i].y, max_y)
    }

    var board [][]byte = make([][]byte, max_y-min_y+1)
    for i := range board {
        board[i] = make([]byte, max_x-min_x +1)
        for j := range board[i] {
            board[i][j] = ' '
        }
    }
    for i := range points {
        var x = points[i].x
        var y = points[i].y
        board[y-min_y][x-min_x] = '#'
    }
    for i := range board {
        fmt.Println(string(board[i]))
    }

}


func Part1(fname string) int{
    var points = ParsePoints(fname)
    MovePoints(points)

    var points_spread = Spread(points)
    var points_prev = ParsePoints(fname)
    var points_spread_prev = Spread(points_prev)
    var converged int
    for points_spread < points_spread_prev {
        MovePoints(points_prev)
        points_spread_prev = Spread(points_prev)
        MovePoints(points)
        points_spread = Spread(points)
        converged ++
    }
    Print(points_prev)
    return converged
}

func main() {
    //Part1("input_test")
    fmt.Printf("Converged: %d\n", Part1("input"))
}
