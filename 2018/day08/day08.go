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

func Parser(iter func(*int) bool,yld chan int,clenaup bool) {
    var children int 
    var metadata_len int 
    iter(&children) 
    iter(&metadata_len)
    for i:= 0; i < children;i++ {
        Parser(iter,yld,false)
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
    go Parser(itr,res,true)
    for ;;{
        i , ok := <- res
        ret += i
        if !ok {
            break
        }
    }

    return ret
}

func main(){
    fmt.Printf("Part 1 %d\n", Part1("input_test"))
    fmt.Printf("Part 1 %d\n", Part1("input"))
}
