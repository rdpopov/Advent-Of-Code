package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)
const (
    Left int = iota
    Center
    Right
)

type Point struct {
    x int
    y int
}
var dirs = [...]Point {
    Point{x:1,  y:0},  // pos x
    Point{x:0,  y:-1},  // pos y
    Point{x:-1, y:0},  // neg x
    Point{x:0,  y:1}, // neg x
}

type Car struct {
    x int
    y int
    crnt_dir int
    dir int
    valid bool
}
// this bellow dowsnt work , plzfix
func (self *Car) Move () {
    self.x += dirs[self.crnt_dir].x
    self.y += dirs[self.crnt_dir].y
}

var directions = [...]int{-1,0,1}
func (self *Car) Turn () {
    self.crnt_dir += directions[self.dir] + 4
    self.crnt_dir %= 4
    self.dir += 1
    self.dir %= 3
}

func (self *Car) Corner (crn rune) {
    if crn == '\\' {
        if self.crnt_dir % 2 == 0{
            self.crnt_dir -= 1
            self.crnt_dir += 4
            self.crnt_dir %= 4
        } else {
            self.crnt_dir += 1
            self.crnt_dir += 4
            self.crnt_dir %= 4
        }
    }
    if crn == '/' {
        if self.crnt_dir % 2 == 0{
            self.crnt_dir += 1
            self.crnt_dir += 4
            self.crnt_dir %= 4
        } else {
            self.crnt_dir -= 1
            self.crnt_dir += 4
            self.crnt_dir %= 4
        }
    }
}
func CarFromRune(car rune,x int ,y int) Car {
    var dir int
    switch car {
    case '>':
        dir = 0
    case '^':
        dir = 1
    case '<':
        dir = 2
    case 'v':
        dir = 3
    }
    return Car{x:x,y:y,crnt_dir:dir,dir:Left,valid: true}
}


func ParsePoints(fname string) ([][]rune, []Car) {
    var f, err = os.Open(fname)
    var cars []Car = make([]Car, 0)
    var grid [][]rune = make([][]rune,0)
    if err!= nil {
        log.Fatal(err)
    }
    defer f.Close()
    var scanner = bufio.NewScanner(f)
    // parse field first
    for scanner.Scan(){
        grid = append(grid, []rune(scanner.Text()))
    }
    for _y,l := range grid {
        for _x,v := range l {
            if v == 'v'  || v == '^'{
                cars = append(cars, CarFromRune(v,_x,_y))
                grid[_y][_x] = '|'
            }
            if v == '<' || v == '>' {
                cars = append(cars, CarFromRune(v,_x,_y))
                grid[_y][_x] = '-'
            }
        }
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
    // PrintField(grid,cars)
    return grid,cars
}

func ValidCars(cars []Car) (int,int) {
    var res int
    var pos int
    for i,v := range cars {
        if v.valid{
            pos = i
            res ++
        }
    }
    return res,pos
}
func PrintField(grid [][]rune,cars []Car) {
    for _y,l := range grid {
        for _x,v := range l {
            var printted = true
            for _,c := range cars {
                if c.x == _x && c.y == _y && c.valid {
                    fmt.Printf("*")
                    printted = false
                }
            }
            if printted {
                fmt.Printf("%s",string(v))
            }
        }
        fmt.Println()
    }
    // for _,c := range cars {
    //     fmt.Println(c)
    // }
}

func CheckCollisions(points []Car,point_idx int) bool{
    for i := range points{
        if i!=point_idx{
            if points[point_idx].x == points[i].x && 
                points[point_idx].valid && 
                points[i].valid &&
                points[point_idx].y == points[i].y {
                points[point_idx].valid = false
                points[i].valid = false
                return true
            }
        }
    }
    return false
}

func MoveCarsPart1(grid [][]rune,points []Car) (bool,int,int) {
    for i:= range points {
        if points[i].valid == false {continue}
        points[i].Move()
        var x = points[i].x; 
        var y = points[i].y;
        if  grid[y][x] == '+' {
            points[i].Turn()
        }
        if grid[y][x] == '/' || grid[y][x] == '\\' {
            points[i].Corner(grid[y][x])
        }
        var has_collision  = CheckCollisions(points,i)
        if has_collision{
            return has_collision,points[i].x,points[i].y
        }
    }
    return false,-1,-1
}

func MoveCarsPart2(grid [][]rune,points []Car) int{
    var ret int = -1
    for i:= range points {
        points[i].Move()
        var x = points[i].x; 
        var y = points[i].y;
        var has_collision  = CheckCollisions(points,i)
        if has_collision{
            var n_valid,last_valid = ValidCars(points)
            if n_valid==1 {
                ret = last_valid
                // points[last_valid].Move()
                // return last_valid
            }
        }
        if  grid[y][x] == '+' {
            points[i].Turn()
        }
        if grid[y][x] == '/' || grid[y][x] == '\\' {
            points[i].Corner(grid[y][x])
        }
    }

    fmt.Println(points[3])
    return ret
}

func Part1(fname string) []int {
    var grid,points = ParsePoints(fname)
    for i:=0;;i++ {
        var has_collision,resx,resy = MoveCarsPart1(grid,points)
        // MoveCars(grid,points)
        if has_collision {
        return []int{resx,resy}
        }
        // PrintField(grid,points)
    }
}

func Part2(fname string) []int {
    var grid,points = ParsePoints(fname)
    for i:=0;;i++ {
        var idx = MoveCarsPart2(grid,points)
        var newPoints []Car = make([]Car,0)
        for _,v:=range points{
            if v.valid{
                newPoints= append(newPoints,v )
            }
        }
        // PrintField(grid,points)
        // fmt.Println(len(newPoints))
        if idx != -1 {
            return []int{points[idx].x,points[idx].y}
        }
    }
    return []int{0,0}
}

func main() {
    // fmt.Printf("Cars: %v \n", Part1("input_test"))
    // fmt.Printf("Cars: %v \n", Part2("input_test_p2"))
    // fmt.Printf("Cars: %v \n", Part1("input"))
    fmt.Printf("Cars: %v \n", Part2("input"))
}
