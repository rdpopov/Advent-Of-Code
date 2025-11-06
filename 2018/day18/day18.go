package main

import (
	"bufio"
	// "bytes"
	// "bytes"
	"time"
	"fmt"
	"log"
	"os"
	// "strings"
)
func assert(a bool, s string) {
	if !a {
		panic(s)
	}
}


func Simulate(field *[][]byte , newField *[][]byte) (bool,int) {
	var tmp = make(map[byte]int); 
	var maxx = len(*field)
	var maxy = len((*field)[0])
	var there_is_change = false

	for x:= 0; x < maxx; x ++ {
		for y := 0; y < maxy; y++ {
			for k := range tmp {
				delete(tmp, k)
			}
			for X := max(x-1, 0); X <= min(x+1, maxx-1); X++ {
				for Y := max(y-1, 0); Y <= min(y+1, maxy-1); Y++ {
					if X == x && Y == y {
						continue
					}
					tmp[(*field)[X][Y]] += 1
				}
			}

			switch (*field)[x][y] {
			case '.':
				{
					if tmp['|'] >= 3 {
						(*newField)[x][y] = '|'
					} else {
						(*newField)[x][y] = '.'
					}
				}
				break
			case '|':
				{
					if tmp['#'] >= 3 {
						(*newField)[x][y] = '#'
					} else {
						(*newField)[x][y] = '|'
					}
				}
				break
			case '#':
				{
					if tmp['#'] >= 1 && tmp['|'] >= 1 {
						(*newField)[x][y] = '#'
					} else {
						(*newField)[x][y] = '.'
					}
				}
				break
			}
			there_is_change = there_is_change || ((*newField)[x][y] != (*field)[x][y])
		}
	}

	return there_is_change,Score(newField)
}

func Score(field *[][]byte) int {
	var woods int = 0
	var yards int = 0
	for x:= 0; x < len(*field); x ++ {
		for y := 0; y < len((*field)[0]); y++ {
			switch (*field)[x][y] {
			case '|':
				woods += 1
				break
			case '#':
				yards += 1
				break
			}
		}
	}
	// fmt.Printf("Woods: %d, Yards: %d\n",woods,yards)
	return woods * yards
}

func Part1(fname string,gens int,dbg bool) []int {
	var file, err = os.Open(fname)
	var res = make([]int,gens)
	if err != nil {
		log.Fatal(err)
	}

	var field [][]byte = make([][]byte, 0);
	var newField [][]byte = make([][]byte, 0);

	defer file.Close()
	var scanner = bufio.NewScanner(file)
	for scanner.Scan() {
		field = append(field, []byte(scanner.Text()))
	}

	newField = make([][]byte, len(field))
	for i := 0; i < len(field); i++ {
		newField[i] = make([]byte,len(field))
		copy(newField[i],field[i])
	}

	chg := true
	score := 0
	for gen := 0; gen < gens; gen++ {

		chg,score = Simulate(&field,&newField)
		res[gen] = score
		field, newField = newField, field
		if chg == false {
			fmt.Println("No changes, stopping early at gen ",gen)
		}

		if !chg {
			break
		}

		if dbg {
			fmt.Print("\033[H\033[2J")
			for x:= 0; x < len(field); x ++ {
				fmt.Printf("%s\n",string(field[x]))
			}
			fmt.Printf("\n")
			time.Sleep(30 * time.Millisecond)
		}

	}


	fmt.Printf("Score after %d gens: %d\n",gens,Score(&field))
	
	return res
}

func Part_helper(fname string,gens int,dbg bool) (int,[]int) {
	var res = Part1(fname,gens,dbg);
	// fmt.Printf("%s",(res))
	var save_i = 0
	var save_ln = 0
	var repeat []int = nil
	var is_repeat bool = false

	for i:=len(res)/2;i<len(res);i-- {
		for ln:=i;ln<len(res);ln++ {
			if res[i] == res[ln] && i != ln {
				save_ln = ln
		        save_i = i
		        is_repeat = true
				for k := i; k<ln -i ; k++ {
					if res[save_i + k] != res[save_ln + k] {
						is_repeat = false
					}
				}
				if is_repeat {
					repeat = make([]int,ln - i)
					for k := 0; k < ln - i; k++ {
						repeat[k] = res[save_i + k]
					}
					return save_i,repeat
				}
			}
		}
	}
	return -1,repeat
}

func Part2(fname string,gens int,dbg bool) int {
	var start, seq = Part_helper(fname,2000,dbg);
	if start == -1{
		fmt.Println("No repetition found")
	} else {
		fmt.Printf("Repetition found at %d with length %d\n",start,len(seq));
		var index = (gens - start -1) % len(seq)
		return seq[index]
	}
	return -1
}

func main() {
	// Part1("ex",10000,true);
	Part1("input",10,false);
	fmt.Printf("%d",Part2("input",1000000000,false));
}
