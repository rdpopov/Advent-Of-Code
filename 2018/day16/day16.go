package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func b2i (a bool) int{
	if a {
		return 1
	}
	return 0
}
type op func([]int,[]int)[]int

type Oper struct {
	f op;
	s string;
    im int;// input mask - which inputs are reisters
}

var map_of_funcs []Oper = []Oper{
	Oper{ f: addr,s:"addr",im: 3 },
	Oper{ f: addi,s:"addi",im: 1 },
	Oper{ f: mulr,s:"mulr",im: 3 },
	Oper{ f: muli,s:"muli",im: 1 },
	Oper{ f: banr,s:"banr",im: 3 },
	Oper{ f: bani,s:"bani",im: 1 },
	Oper{ f: borr,s:"borr",im: 3 },
	Oper{ f: bori,s:"bori",im: 1 },
	Oper{ f: setr,s:"setr",im: 1 },
	Oper{ f: seti,s:"seti",im: 0 },
	Oper{ f: gtir,s:"gtir",im: 2 },
	Oper{ f: gtri,s:"gtri",im: 1 },
	Oper{ f: gtrr,s:"gtrr",im: 3 },
	Oper{ f: eqir,s:"eqir",im: 2 },
	Oper{ f: eqri,s:"eqri",im: 1 },
	Oper{ f: eqrr,s:"eqrr",im: 3 },
}

func process_complex(s string) (ret []int){
	ret = make([]int,4)
	var drop string
	fmt.Sscanf(s,"%s [%d, %d, %d, %d]",&drop,&ret[0],&ret[1],&ret[2],&ret[3])
	return ret
}

func process(s string) (ret []int){
	ret = make([]int,4)
	fmt.Sscanf(s,"%d %d %d %d",&ret[0],&ret[1],&ret[2],&ret[3])
	return ret
}

func make_instruction(s []string) (ret [][]int){
	ret = make([][]int,0)
	ret = append(ret, process_complex(s[0]))
	ret = append(ret, process(s[1]))
	ret = append(ret, process_complex(s[2]))
	return ret
}

func looks(f [][]int, im int, res []int) bool {
	var idx = f[1][3]
	var r = true
	if (1 & im) == 1 {
		if !(f[1][1] == f[1][3]) {
			r = r && f[2][f[1][1]] == res[f[1][1]]
		}
	}

	if (2 & im) == 2 {
		if !(f[1][2] == f[1][3]) {
			r = r && f[2][f[1][2]] == res[f[1][2]]
		}
	}
	// fmt.Printf( "idx: %d value %d vec %v\n" ,idx, res[idx],  res  )
	// fmt.Printf( "idx: %d value %d vec %v\n" ,idx, f[2][idx],f[2]  )
	// fmt.Println(res[idx] == f[2][idx])
	return res[idx] == f[2][idx]
}

 // bitmask of the functions
func try_function(f [][]int) int {
	var res = 0
	for i := 0; i < len(map_of_funcs); i++ {
		var tmp = map_of_funcs[i].f(f[1],f[0])
		// fmt.Println(map_of_funcs[i].s,f,tmp)
		if looks(f,map_of_funcs[i].im,tmp) {
			res = res | (1 << i)
		}
	}
	return res
}

func countBits(inp int) (res int) {
	res = 0 
	for i:=inp; i > 0 ;i=i>>1 {
		if i &1 == 1{
			res ++
		}
	}
	return res
}

func Part1(fname string) {
	var file, err = os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}

	var board [][]string = make([][]string, 0)
	var program [][][]int = make([][][]int, 0)

	board = append(board, make([]string, 0))
	defer file.Close()
	var scanner = bufio.NewScanner(file)
	var instr = 0
	for scanner.Scan() {
		if scanner.Text() == "" {
			board = append(board, make([]string, 0))
			instr ++
		} else {
			board[instr] = append(board[instr], scanner.Text())
		}
	}
	// var program_celaned = board[:len(board)-3]
	var program_celaned = board
	for i:=0; i < len(program_celaned);i++{
		// fmt.Println(program_celaned[i])
		program = append(program, make_instruction(program_celaned[i]))
	}
	var res = 0
	for i:=0; i < len(program);i++{ 
		if countBits(try_function(program[i])) > 2 {
			res ++ 
		}
	}
	fmt.Println(res)
}

func l2(a int) int{
	var i =0
	for i=0;a >0;i++{
		a = a >> 1
	}
	return i
}

func createTable(masks []int) []int {
	res :=  make([]int,len(masks))
	for i:=0;i< len(masks);i++ {
		res[i] = l2(masks[i]) - 1
	}
	return res
}

func Part2_prepare_functions(fname string) []int{
	var file, err = os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}

	var board [][]string = make([][]string, 0)
	var program [][][]int = make([][][]int, 0)

	board = append(board, make([]string, 0))
	defer file.Close()
	var scanner = bufio.NewScanner(file)
	var instr = 0
	for scanner.Scan() {
		if scanner.Text() == "" {
			board = append(board, make([]string, 0))
			instr ++
		} else {
			board[instr] = append(board[instr], scanner.Text())
		}
	}
	var program_celaned = board
	for i:=0; i < len(program_celaned);i++{
		// fmt.Println(program_celaned[i])
		program = append(program, make_instruction(program_celaned[i]))
	}
	var res = make([]int,16)
	
	for i:=0; i < len(program);i++{ 
		var mask = try_function(program[i])
		if res[program[i][1][0]] != 0 {
			 res[program[i][1][0]] = res[program[i][1][0]] & mask
		} else {
			 res[program[i][1][0]]  =  mask
		}
	}

	var used []int = make([]int,len(res))
	var one = len(res)
	var next = 0
	for one > 0 {
		for i:=0; i < len(res);i++{ 
			if used[i] == 0 && countBits(res[i]) == 1 {
				for j:=0; j < len(res);j++{ 
					if j != i {
						res[j] = res[j] & ^res[i]
					}
				}
				used[i] = res[i]
				one --
			}
		}
		if next == one {
			for i:=0; i < len(res);i++{ 
				fmt.Println(i, countBits(res[i]))
			}
			break
		}
		next = one
	}

	return createTable(res)
}

func Part2(fname_func string,fname_prg string){
	var tr_table = Part2_prepare_functions(fname_func)
	var program[][]int = make([][]int, 0)
	var file,err = os.Open(fname_prg)
	defer file.Close()

	if err != nil {
		log.Fatal(err)
	}
	var sc = bufio.NewScanner(file)
	for sc.Scan() {
		program = append(program, process(sc.Text()))
	}
	var regs = make([]int,4)
	for i:=0;i< len(program);i++ {
		regs = map_of_funcs[tr_table[program[i][0]]].f(program[i],regs)
	}
	fmt.Println(regs)
}

func main() {
	// var a = []int{0,1,2,3}
	// var b = []int{2,3,4,5}

	// fmt.Println(a,b)
	// fmt.Println(addr(a,b))
	// fmt.Println(a,b)
	// Part1("inp")
	// Part1("input_p1")
	Part2("input_p1","input_p2")
	// fmt.Println(countBits(3))
}
