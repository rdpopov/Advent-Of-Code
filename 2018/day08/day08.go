package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

// type File struct {
//     children []*File
//     parent   *File
//     metadata []int
// }

// type IFile interface {
//     NewFile(parent *File) File
// //     Next(f File) *File
// //     AddChildren(nch int)
// //     GetMetadataSum(f File) int
// }
// func (f File) NewFile(children int) File {
//     var res = File{parent: &f,children: make([]*File, children)}
//     if &f != nil {
//         f.children = append(f.children, &res)
//     }
//     return res
// }
func ItrIntsInFile(f *os.File) (func(*int) bool,error) {
    var scanner = bufio.NewScanner(f)
    scanner.Split(bufio.ScanWords)
    scanner.Scan()
    return func(c *int) bool {
        if err:= scanner.Err(); err!= nil {
            log.Fatal(err)
            return false
        } else {
            fmt.Sscanf(scanner.Text(),"%d",c)
            return scanner.Scan()
        }
    },nil
}

func ParserPart1(iter func(*int) bool,yld chan int,clenaup bool) {
    var children int 
    var metadata_len int 
    iter(&children) 
    iter(&metadata_len)
    for i:= 0; i < children;i++ {
        ParserPart1(iter,yld,false)
    }
    for i:= 0; i < metadata_len;i++ {
        iter(&children)
        yld <- children
    }
    if clenaup {
        close(yld)
    }
}

func Part1 (fname string) int {
    var ret int
    f,err:= os.Open(fname)
    if err != nil {
        log.Fatal("Could not open file")
    }

    var itr,err2 = ItrIntsInFile(f)
    if err2!= nil {
        log.Fatal(err2)
    }
    var res = make(chan int)
    go ParserPart1(itr,res,true)
    for ;;{
        i , ok := <- res
        ret += i
        if !ok {
            break
        }
    }

    return ret
}

func Part2 (fname string) int {
    f,err:= os.Open(fname)
    if err != nil {
        log.Fatal("Could not open file")
    }

    var itr,err2 = ItrIntsInFile(f)
    if err2!= nil {
        log.Fatal(err2)
    }
    var res = ParserPart2(itr)
    return res
}

func ParserPart2(iter func(*int) bool) int {
    var children int
    var metadata_len int
    iter(&children)
    iter(&metadata_len)
    if children > 0 {
        var res []int = make([]int, children)
        for i:= 0; i < children;i++ {
            res[i] = ParserPart2(iter)
        }
        var children_value int
        var crnt_value int
        for i:= 0; i < metadata_len;i++ {
            iter(&crnt_value)
            crnt_value --
            if  0 <= crnt_value && crnt_value < children {
                children_value += res[crnt_value]
            }
        }
        return children_value
    } else {
        var res int
        for i:= 0; i < metadata_len;i++ {
            iter(&children)
            res += children
        }
        return res
    }
}

func main(){
    fmt.Printf("Part 1 %d\n", Part1("input_test"))
    fmt.Printf("Part 1 %d\n", Part1("input"))
    fmt.Printf("Part 2 %d\n", Part2("input_test"))
    fmt.Printf("Part 2 %d\n", Part2("input"))
}
