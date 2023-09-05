package main

import (
	"fmt"
	"time"
)
type Square struct
{
    x,y int
    size int
    value int
}


func calcPower(x int, y int, serial int) int {
    return ((((( x+10 ) * y + serial ) % 1000 ) * (x + 10)) % 1000) / 100 - 5
}

// func calcSquare(x int, y int, serial int) int {
//     if x < 1 || y < 1 || 299 < x || 299 < y {
//         return -45
//     } else {
//         var ret int
//         ret+=calcPower(x, y , serial)
//         ret+=calcPower(x+1, y+1 , serial)
//         ret+=calcPower(x, y+1 , serial)
//         ret+=calcPower(x+1, y , serial)
//         ret+=calcPower(x-1, y-1 , serial)
//         ret+=calcPower(x, y-1 , serial)
//         ret+=calcPower(x-1, y , serial)
//         ret+=calcPower(x-1, y+1 , serial)
//         ret+=calcPower(x+1, y-1 , serial)
//         return ret
//     }
// }

func calcSquareSize(x int, y int, serial int,size int) (int,bool) {
    if x < 0 || y < 0 || 299 < x+size || 299 < y+size {
        return 0,false
    } else {
        var ret int
        for i:=0;i<size;i++ {
            for j:=0;j<size;j++ {
                ret+=calcPower(x+i, y+j , serial)
            }
        }
        return ret,true
    }
}

func Part1(serial int) Square {
    return findBestgSquareSize(serial,3)
}

func findBestgSquareSize(serial int,sqSize int) Square {
    var max_val,_ = calcSquareSize(2,2,serial,sqSize)
    var max_x = 2
    var max_y = 2
    var loop_upper_bound = 300 - sqSize + 1

    for i:=1;i<loop_upper_bound;i++{
        for j:=1;j<loop_upper_bound;j++{
            var crnt_value,_ = calcSquareSize(i,j,serial,sqSize)
            if max_val < crnt_value {
                max_x,max_y = i,j
                max_val = crnt_value
            }
        }
    }
    return Square{ max_x,max_y,sqSize,max_val }
}
func gruntWorker(sqSize int, serial int,output chan Square) {
    output <- findBestgSquareSize(serial,sqSize)
}

func Part2(serial int) Square {
    var output chan Square = make(chan Square,300)
    var recieved int
    var max_sq Square
    for i:=1; i<=300; i++ {
        go gruntWorker(i,serial,output)
    }

    for {
        select {
        case i:= <- output:
            recieved++
            if i.value > max_sq.value {
                max_sq = i
            }
        default:
            if recieved == 299{
                return max_sq
            }
            time.Sleep(1)
        }
    }
}


func main() {
    fmt.Printf("Part 1 %v \n", Part1(8979))
    fmt.Printf("Part 2 %v \n", Part2(8979))
}
