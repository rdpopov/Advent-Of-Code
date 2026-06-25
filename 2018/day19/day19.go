package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
)

func assert(a bool, s string) {
	if !a {
		panic(s)
	}
}


func ExecuteInstruction(regs []int,instr [][4]int ,ip int) () {
	// instr: [instr , x ,y, res]
	var line [4]int = instr[regs[ip]]
	switch line[0] {
	case 1://"addr":
		regs[line[3]] = regs[line[1]] + regs[line[2]]
	case 2: //"addi":
		regs[line[3]] = regs[line[1]] + line[2]
	case 3: //"seti":
		regs[line[3]] = line[1]
	case 4: //"setr":
		regs[line[3]] = regs[line[1]]
	case 5: //eqrr
		if regs[line[1]] == regs[line[2]] {
			regs[line[3]] = 1
		} else  {
			regs[line[3]] = 0
		}
	case 6: //gtrr
		if regs[line[1]] > regs[line[2]] {
			regs[line[3]] = 1
		} else  {
			regs[line[3]] = 0
		}
	case 7: //muli
		regs[line[3]] = regs[line[1]] * line[2]
	case 8: //mulr
		regs[line[3]] = regs[line[1]] * regs[line[2]]
	}
	regs[ip] += 1
}

func Part1(fname string) int {
	var file, err = os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var program [][4]int = make([][4]int, 0) ;
	var registers = make([]int, 6)
	var ip = 0
	var m map[string]int = make(map[string] int)

	m["addr"] = 1
	m["addi"] = 2
	m["seti"] = 3
	m["setr"] = 4
	m["eqrr"] = 5
	m["gtrr"] = 6
	m["muli"] = 7
	m["mulr"] = 8

	var scanner = bufio.NewScanner(file)
	for scanner.Scan() {
		if scanner.Text()[0] == '#' {
			fmt.Sscanf(scanner.Text(), "#ip %d", &ip)
			continue
		} else {
			var _instr string
			var instr [4]int
			fmt.Sscanf(scanner.Text(), "%s %d %d %d", &_instr, &instr[1], &instr[2], &instr[3])
			instr[0] = m[_instr]
			program = append(program, instr)
		}
	}
	
	for registers[ip] < len(program) {
		ExecuteInstruction(registers,program,ip)
	}

	return registers[0]
}
func Factorize(num int) int {
	fmt.Println(math.Sqrt(float64(num)))
	var count int = 0
	var nm int = int(math.Sqrt(float64(num)))

	for i:=1;i<nm;i+=1 {
		if num % i == 0 {
			count +=i
			count += (num/i)
		}
	}

	return count
}

func Part2(fname string) int {
	var file, err = os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var program [][4]int = make([][4]int, 0) ;
	var registers = make([]int, 6)
	registers[0]=1
	var ip = 0
	var m map[string]int = make(map[string] int)
	m["addr"] = 1
	m["addi"] = 2
	m["seti"] = 3
	m["setr"] = 4
	m["eqrr"] = 5
	m["gtrr"] = 6
	m["muli"] = 7
	m["mulr"] = 8

	var mr map[int]string = make(map[int] string)
	mr[1] = "addr"
	mr[2] = "addi"
	mr[3] = "seti"
	mr[4] = "setr"
	mr[5] = "eqrr"
	mr[6] = "gtrr"
	mr[7] = "muli"
	mr[8] = "mulr"

	var scanner = bufio.NewScanner(file)
	for scanner.Scan() {
		if scanner.Text()[0] == '#' {
			fmt.Sscanf(scanner.Text(), "#ip %d", &ip)
			continue
		} else {
			var _instr string
			var instr [4]int
			fmt.Sscanf(scanner.Text(), "%s %d %d %d", &_instr, &instr[1], &instr[2], &instr[3])
			instr[0] = m[_instr]
			program = append(program, instr)
		}
	}
	// var rdr = bufio.NewReader(os.Stdin)
	var heatmap []int =  make([]int,len(program))
	
	//after runningalmost once: [0 0 0 10550400 35 10551345]
	// for registers[ip] < len(program) {
	
	for a:=0;a < len(program);a+=1 {
	// for a:=0;a < 10551345*20;a+=1 {
		heatmap[registers[ip]] +=1
		ExecuteInstruction(registers,program,ip)
	}
	// upto len program times should have primed it
	// find the count of all the pars of numbers upto  sqrt  whatever is in register 5 an

	for i:=0;i<len(heatmap);i+=1{
		fmt.Println(i,"\t",mr[program[i][0]],program[i][1:], heatmap[i])
	}
	// The program goes up from 1 and finds all of the 2 pair numbers that
	// multiply to 10551345
	fmt.Println(registers)

	return Factorize(registers[5])
}

func main() {
	// fmt.Println(Part1("ex"));
	// fmt.Println(Part1("input"));
	fmt.Println(Part2("input"));
}
