package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)
func abs(x int) int {
    if x > -1 {
        return x
    }
    return -x
}

const X = 0
const Y = 1

func manhatan(this, other [2]int) int {
    return abs(this[X] - other[X]) + abs(this[Y] - other[Y])
}

func min_array(arr []int) int {
    var ret int = arr[0]
    for _,v:= range arr {
        ret = min(ret,v)
    }
    return ret
}
func sum_array(arr []int) int {
    var ret int = 0
    for _,v:= range arr {
        ret += v
    }
    return ret
}

func count(arr []int,value int) []int {
    var cnt []int = make([]int,0)
    for i,v:= range arr {
        if value == v {
            cnt = append(cnt, i)
        }
    }
    return cnt
}

func best_manhatan(points [][2]int, target [2]int,store []int) []int {
    for i,v := range points {
        store[i] = manhatan(v,target)
    }
    var min_value = min_array(store)
    // fmt.Println(target,min_value,count(store,min_value))
    return count(store,min_value)
}

func Part1(fname string) int{
    var f,err = os.Open(fname)
    if err != nil { log.Fatal(err) }
    var scanner = bufio.NewScanner(f)
    var points [][2]int
    var origin [2]int
    var edge [2]int
    for scanner.Scan() {
        var parsed = strings.FieldsFunc(scanner.Text(),func(r rune) bool { return r == ',' })
        var point [2]int
        fmt.Sscan(parsed[0],&point[0])
        fmt.Sscan(parsed[1],&point[1])

        origin[X] = min(origin[X],point[X])
        origin[Y] = min(origin[Y],point[Y])
        edge[X] = max(edge[X],point[X])
        edge[Y] = max(edge[Y],point[Y])

        points = append(points, point)
    }

    var infinite = points_to_ignore(points,origin,edge)
    var points_table = countPoints_part1(points,origin,edge)
    if err:=scanner.Err(); err != nil { log.Fatal(err) }
    return maxInNonInfinite(points_table,infinite)
}

func Part2(fname string,limit int) int{
    var f,err = os.Open(fname)
    if err != nil { log.Fatal(err) }
    var scanner = bufio.NewScanner(f)
    var points [][2]int
    var origin [2]int
    var edge [2]int
    for scanner.Scan() {
        var parsed = strings.FieldsFunc(scanner.Text(),func(r rune) bool { return r == ',' })
        var point [2]int
        fmt.Sscan(parsed[0],&point[0])
        fmt.Sscan(parsed[1],&point[1])

        origin[X] = min(origin[X],point[X])
        origin[Y] = min(origin[Y],point[Y])
        edge[X] = max(edge[X],point[X])
        edge[Y] = max(edge[Y],point[Y])

        points = append(points, point)
    }

    if err:=scanner.Err(); err != nil { log.Fatal(err) }
    return countPoints_part2(points,origin,edge,limit)
}

func points_to_ignore(points [][2]int, origin, edge [2]int) []bool {
    var res = make([]bool, len(points))
    for i :=range res {
        res[i] = true
    }
    var store = make([]int,len(points)) //intermediate storage

    for x:=origin[X]; x<=edge[X];x++ {
        var closest_idx_es = best_manhatan(points,[2]int{x,origin[Y]},store)
        if len(closest_idx_es) == 1 {
            res[closest_idx_es[0]] = false
        }
        var closest_idx_es_ = best_manhatan(points,[2]int{x,edge[Y]},store)
        if len(closest_idx_es_) == 1 {
            res[closest_idx_es_[0]] = false
        }
    }

    for y:=origin[Y]; y<=edge[Y];y++ {
        var closest_idx_es = best_manhatan(points,[2]int{origin[X],y},store)
        if len(closest_idx_es) == 1 {
            res[closest_idx_es[0]] = false
        }
        var closest_idx_es_ = best_manhatan(points,[2]int{edge[X],y},store)
        if len(closest_idx_es_) == 1 {
            res[closest_idx_es_[0]] = false
        }
    }
    return res
}
func countPoints_part1(points [][2]int, origin, edge [2]int) []int {
    var res = make([]int,len(points))
    var store = make([]int,len(points)) //intermediate storage
    for x:=origin[X]; x<=edge[X];x++ {
        for y:=origin[Y]; y<=edge[Y];y++ {
            var closest_idx_es = best_manhatan(points,[2]int{x,y},store)
            if len(closest_idx_es) == 1 {
                res[closest_idx_es[0]] += 1
            }
        }
    }
    return res
}
func countPoints_part2(points [][2]int, origin, edge [2]int,limit int) int {
    var res int
    var store = make([]int,len(points)) //intermediate storage
    for x:=origin[X]; x<=edge[X];x++ {
        for y:=origin[Y]; y<=edge[Y];y++ {
            var _ = best_manhatan(points,[2]int{x,y},store)
            if sum_array(store) < limit {
                res += 1
            }
        }
    }
    return res
}
func maxInNonInfinite(points_table []int, infinite []bool) int {
    var res int
    for i := range points_table {
        if infinite[i] {
            res = max(res,points_table[i])
        }

    }
    return res
}

func main() {
    fmt.Printf("Part 1 %d\n",Part1("input_test"))
    fmt.Printf("Part 1 %d\n",Part1("input"))
    fmt.Printf("Part 2 %d\n",Part2("input_test",32))
    fmt.Printf("Part 2 %d\n",Part2("input",10000))
}
