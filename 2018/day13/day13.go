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
    Point{x:0,  y:1},  // pos y
    Point{x:-1, y:0},  // neg x
    Point{x:0,  y:-1}, // neg x
}

type Car struct {
    x int
    y int
    crnt_dir int
    dir int
}
// this bellow dowsnt work , plz fix
func (self *Car) Move () {
    self.x += dirs[self.crnt_dir].x
    self.y += dirs[self.crnt_dir].y
}

func (self *Car) Turn () {
    self.crnt_dir += self.dir
    self.crnt_dir %= 4
    self.dir += 1
    self.dir %= 3
}

func (self *Car) Corner (crn rune) {
    if crn == '\\' {
        if self.crnt_dir % 2 == 0{
            self.crnt_dir += 1
            self.crnt_dir %= 4
        } else {
            self.crnt_dir -= 1
            self.crnt_dir %= 4
        }
    }
    if crn == '/' {
        if self.crnt_dir % 2 == 0{
            self.crnt_dir -= 1
            self.crnt_dir += 4
            self.crnt_dir %= 4
        } else {
            self.crnt_dir += 1
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
    return Car{x:x,y:y,crnt_dir:dir,dir:Left}
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
    PrintField(grid,cars)
    return grid,cars
}

func PrintField(grid [][]rune,cars []Car) {
    for _y,l := range grid {
        for _x,v := range l {
            var printted = true
            for _,c := range cars {
                if c.x == _x && c.y == _y {
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
    for _,c := range cars {
        fmt.Println(c)
    }
}

func CheckCollisions(points []Car,point_idx int) (bool,int,int) {
    for i := range points{
        if i!=point_idx{
            if points[point_idx].x == points[i].x &&
            points[point_idx].y == points[i].y {
                return true,points[i].x,points[i].x
            }
        }
    }
    return false,-1,-1
}

func MoveCars(grid [][]rune,points []Car) (bool,int,int) {
    for i:= range points {
        points[i].Move()
        var x = points[i].x; 
        var y = points[i].y;
        if  grid[y][x] == '+' {
            points[i].Turn()
        }
        if grid[y][x] == '/' || grid[y][x] == '\\' {
            points[i].Corner(grid[y][x])
        }
        var has_collision,resx,resy  = CheckCollisions(points,i)
        if has_collision{
            return has_collision,resx,resy
        }
    }
    return false,-1,-1
}

func Part1(fname string) (int,int) {
    var grid,points = ParsePoints(fname)
    for i:=0;i<3;i++ {
        var has_collision,resx,resy = MoveCars(grid,points)
        if has_collision{
            return resx,resy
        }
        PrintField(grid,points)
    }
    return -1,-1
}

func main() {
    var x,y = Part1("input_test")
    fmt.Printf("Cars: %d\n", x,y)
}
