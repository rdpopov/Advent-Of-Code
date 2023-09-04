package main

import (
	"fmt"
)


func calcPower(x int, y int, serial int) int {
    return ((((( x+10 ) * y + serial ) % 1000 ) * (x + 10)) % 1000) / 100 - 5
}

func calcSquare(x int, y int, serial int) int {
    if x < 1 || y < 1 || 299 < x || 299 < y {
        return -45
    } else {
        var ret int
        ret+=calcPower(x, y , serial)
        ret+=calcPower(x+1, y+1 , serial)
        ret+=calcPower(x, y+1 , serial)
        ret+=calcPower(x+1, y , serial)
        ret+=calcPower(x-1, y-1 , serial)
        ret+=calcPower(x, y-1 , serial)
        ret+=calcPower(x-1, y , serial)
        ret+=calcPower(x-1, y+1 , serial)
        ret+=calcPower(x+1, y-1 , serial)
        return ret
    }
}

func calcSquareSize(x int, y int, serial int,size int) (int,bool) {
    if x-size < 1 || y-size < 1 || 299 < x+size || 299 < y+size {
        return 0,false
    } else {
        var ret int
        ret+=calcPower(x, y , serial)
        for i:=2;i<;i++{
        return ret,true
    }
}

func Part1(serial int) (int,int) {
    var max_val = calcSquare(2,2,serial)
    var max_x = 2
    var max_y = 2

    for i:=2;i<300;i++{
        for j:=2;j<300;j++{
            var crnt_value = calcSquare(i,j,serial)
            if max_val < crnt_value {
                max_x,max_y = i,j
                max_val = crnt_value
            }
        }
    }
    return max_x-1,max_y-1
}

func main() {
    // fmt.Println(calcPower(3,5,8))
    // fmt.Println(calcPower(122,79,57))
    // fmt.Println(calcPower(217,196,39))
    // fmt.Println(calcPower(101,153,71))
    var x,y = Part1(8979)
    fmt.Printf("max score %d,%d \n", x,y)
}
