package main
import (
	"bufio"
	"fmt"
	"sort"
	"log"
	"os"
)
type small struct{
	x int
	y int
	atk int
	hp int
	letter byte
}

func count_elves(gnomes []small) int {
	var ret =0
	for  i := 0; i<len(gnomes); i++ {
		if gnomes[i].letter =='E'{
			ret ++
		}
	}
	return ret
}

func Parse_gnomes(board [][]byte,elf_attack int) []small {
	var ret = make([]small, 0)
	for  i := 0; i<len(board); i++ {
		for  j := 0; j<len(board[i]); j++ {
			if board[i][j] == 'E' {
				ret = append(ret, small{j,i,elf_attack,200,board[i][j]})
			}
			if board[i][j] == 'G' {
				ret = append(ret, small{j,i,3,200,board[i][j]})
			}
		}
	}
	return ret
}

func printBoard(board [][]byte,mask_board [][]int){
	var ly = len(board)
	for  i := 0; i<ly; i++ {
		// fmt.Printf("%s\n",board[i])
		fmt.Printf("%s %v\n",board[i],mask_board[i])
	}
	fmt.Printf("\n")
}

func printGnomes(gnomes []small){
	var ly = len(gnomes)
	for  i := 0; i<ly; i++ {
		var g = gnomes[i]
		fmt.Printf("%d %c %d %d %d %d\n",i,g.letter,g.hp,g.x,g.y,has_in_range(gnomes,i))
		// fmt.Printf("%s %v\n",board[i],mask_board[i])
	}
	fmt.Printf("\n")
}

func clear(board [][]int) {
	for  i := 0; i<len(board); i++ {
		for  j := 0; j<len(board[i]); j++ {
			board[i][j] = 0
		}
	}
}
func is_enemy(current byte, other byte ) bool {
	if current == 'E' {
		return other == 'G'

	}
	if current == 'G' {
		return other == 'E'
	}
	return false
}

func btw(x int ,a int , b int) bool {
	return a < x && x < b
}

func minimum_square_xy(mask_board [][]int,i int ,j int) (int,int){
	var m = 1025
	var x,y int = i, j

	var lx,ly int = len(mask_board[0]), len(mask_board)
	
	if btw(i-1,-1,ly) && btw(mask_board[i-1][j],0,m)  {
		m = mask_board[i-1][j]
		x,y = i-1, j
	}
	// fmt.Printf("%d %d -> ",i,j)
	if btw(j-1,-1,lx) && btw(mask_board[i][j-1],0,m)  {
		m = mask_board[i][j-1]
		x,y = i, j-1
	}
	if btw(j+1,-1,lx) && btw(mask_board[i][j+1],0,m)  {
		m = mask_board[i][j+1]
		x,y = i, j+1
	}
	if btw(i+1,-1,ly) && btw(mask_board[i+1][j],0,m)  {
		m = mask_board[i+1][j]
		x,y = i+1, j
	}
	return x,y
}

func fill_board(board [][]byte,mask_board[][]int,gnomes []small, move_idx int) bool {
	var change bool = true
	var ly = len(board)
	var lx = len(board[0])
	var new_value int = 0
	var targets = false
	mask_board[gnomes[move_idx].y][gnomes[move_idx].x] = 1

	for change {
		change = false
		for  i := 0; i<ly; i++ {
			for  j := 0; j<lx; j++ {
				if board[i][j] == '.' && mask_board[i][j] == 0 {
					new_value = 1024
					if i > 1 && mask_board[i-1][j] > 0 && mask_board[i-1][j] < 1024 { // up
						new_value = min(mask_board[i-1][j],new_value)
					}
					if i < ly-1 && mask_board[i+1][j] > 0 && mask_board[i+1][j]  < 1024 { // down
						new_value = min(mask_board[i+1][j],new_value)
					}
					if j > 1 && mask_board[i][j-1] > 0 && mask_board[i][j-1] < 1024 { // left 
						new_value = min(mask_board[i][j-1],new_value)
					}
					if j < lx-1 && mask_board[i][j+1] > 0 && mask_board[i][j+1] < 1024 { // right
						new_value = min(mask_board[i][j+1],new_value)
					}
					if new_value != 1024 {
						var gob = 'G' == board[i-1][j] || 'G' ==board[i+1][j] || 'G' ==board[i][j-1] || 'G' ==board[i][j+1]
						var elves = 'E' ==board[i-1][j] || 'E' ==board[i+1][j] || 'E' ==board[i][j-1] || 'E' ==board[i][j+1]
						if gnomes[move_idx].letter == 'E'{
							targets = targets ||  gob;
						}
						if gnomes[move_idx].letter == 'G'{
							targets = targets || elves;
						}
						change = true
						mask_board[i][j] = new_value +1
					}
				} else if is_enemy(gnomes[move_idx].letter,board[i][j]) && mask_board[i][j] == 0 {
					mask_board[i][j] = 1025
				}
			}
		}
	}
	var mini,minj int = 0,0
	var minimum = 1025

	if targets {
		for  i := 0; i<ly; i++ {
			for  j := 0; j<lx; j++ {
				if mask_board[i][j] == 1025 {
					var mi,mj = minimum_square_xy(mask_board,i,j)
					// fmt.Printf("%d %d -> %d %d\n",mini,minj,mi,mj)
					if 0 < mask_board[mi][mj] && mask_board[mi][mj] < minimum { // no overriding since reading order
						minimum = mask_board[mi][mj]
						mini,minj = mi,mj
					}
				}
			}
		}
		// fmt.Printf("%d\n",minimum)
		for i:=minimum; i > 2; i -- {
			// fmt.Printf("%d %d\n",mini,minj)
			mini,minj = minimum_square_xy(mask_board,mini,minj)
		}

		mask_board[mini][minj] = 1
	}
// if move_idx == 3 {
// 	printBoard(board,mask_board)
// 	fmt.Print(targets)
// }
	return targets || has_in_range(gnomes,move_idx) > -1
}

func min_dist(board [][]byte,mask_board [][]int,gnome small) (int, int, int) {
	var i = gnome.y
	var j = gnome.x

	var ret_i = gnome.y
	var ret_j = gnome.x
	var ly = len(board)
	var lx = len(board[0])

	var new_value = 1024

	if i < ly-1 && mask_board[i+1][j]  > 0{ // down
		ret_i, ret_j = i+1,  j
		new_value = min(mask_board[i+1][j],new_value)
	}
	if j > 1 && mask_board[i][j-1] > 0 { // left 
		ret_i, ret_j = i,  j-1
		new_value = min(mask_board[i][j-1],new_value)
	}
	if j < lx-1 && mask_board[i][j+1] > 0 { // right
		ret_i, ret_j = i,  j+1
		new_value = min(mask_board[i][j+1],new_value)
	}
	if i > 1 && mask_board[i-1][j] > 0{ // up
		ret_i, ret_j = i-1,  j
		new_value = min(mask_board[i-1][j],new_value)
	}
	if new_value == 1024 {
		new_value = -1
	}
	return new_value,ret_i,ret_j
}

func has_in_range(gnomes []small, move_idx int) int {
	var a = gnomes[move_idx]
	for i := 0 ; i < len(gnomes); i++ {
		if gnomes[i].letter != a.letter {
			var t = gnomes[i]
			if (a.x == t.x-1 && a.y == t.y) || (a.x == t.x+1 && a.y == t.y) || (a.x == t.x   && a.y == t.y-1) || (a.x == t.x   && a.y == t.y+1) {
				if t.hp > 0 {
					return i
				}
			}
		}
	}
	return -1
}

func next_move(mask_board [][]int,gnome small) (int, int) {
	var y,x = minimum_square_xy(mask_board,gnome.y,gnome.x)
	return x,y
}

func move_gnome(board [][]byte,mask_board [][]int,gnomes *[]small, move_idx int) bool {
	if has_in_range(*gnomes,move_idx) > -1 {
		return false
	}
	var new_x,new_y = next_move(mask_board,(*gnomes)[move_idx])
	board[(*gnomes)[move_idx].y][(*gnomes)[move_idx].x] = '.'
	board[new_y][new_x] = (*gnomes)[move_idx].letter
	(*gnomes)[move_idx].x = new_x
	(*gnomes)[move_idx].y = new_y

	if has_in_range(*gnomes,move_idx) > -1 {
		return false
	}
	return true
}
func remove_idx(gnomes []small, remove_idx int) []small{
	return append(gnomes[:remove_idx],gnomes[remove_idx+1:]...)
}

func combat(board [][]byte,mask_board [][]int, gnomes *[]small, move_idx int) int{
	var a = (*gnomes)[move_idx]
	var target = -1
	var ts_max_hp = 100000
	for i := 0 ; i < len((*gnomes)); i++{
		if (*gnomes)[i].letter != a.letter  {
			var t = (*gnomes)[i]
			if (a.x == t.x-1 && a.y == t.y) || (a.x == t.x+1 && a.y == t.y) || (a.x == t.x   && a.y == t.y-1) || (a.x == t.x   && a.y == t.y+1)  {
				if (ts_max_hp > t.hp && t.hp > 0 ){
					ts_max_hp = t.hp
					target = i
				}
			}
		}
	}

	// fmt.Printf("%c %d %d %d -> ",(*gnomes)[move_idx].letter,(*gnomes)[move_idx].hp,(*gnomes)[move_idx].x,(*gnomes)[move_idx].y)
	// fmt.Printf("%c %d %d %d\n",  (*gnomes)[target].letter,  (*gnomes)[target].hp,  (*gnomes)[target].x,(*gnomes)[target].y)

	if target > -1 {
		(*gnomes)[target].hp = (*gnomes)[target].hp - a.atk

		if (*gnomes)[target].hp <= 0 {
			// fmt.Println(move_idx,target,(*gnomes)[target].hp)
			board[(*gnomes)[target].y][(*gnomes)[target].x] = '.'

			(*gnomes) = remove_idx((*gnomes),target)
			// fmt.Printf("%d killed %d\n",move_idx,target)

			if target > move_idx {
				return 1
			}
			return 0
		}
	} else{
		panic("What do?")
	}
	return 1
}

func Move_gnome(board [][]byte,mask_board [][]int,gnomes *[]small, move_idx int) int {
	clear(mask_board)

	if fill_board(board,mask_board,*gnomes,move_idx) {
		// fmt.Printf("Combat for %v, with idx %d -> %d \n",(*gnomes)[move_idx],move_idx, has_in_range((*gnomes),move_idx))
		if !move_gnome(board,mask_board,gnomes,move_idx) {
			return combat(board,mask_board,gnomes,move_idx)
		}
	}
	return 1
}

func is_won(gnomes []small) bool{
	var res = true
	for i := 1; i < len(gnomes); i++ {
		res = res && (gnomes[i-1].letter == gnomes[i].letter)
	}
	return res
}

func result(gnomes []small,rnd int) int{
	var res = 0
	for i := 0; i < len(gnomes); i++ {
		res += gnomes[i].hp
	}
	fmt.Printf("%d * %d\n", rnd,res)
	return res*rnd
}

func Round(board [][]byte,mask_board [][]int,gnomes *[]small) int  {
	var rnd  = 0
	for k:=0;;k++ {
		for i := 0; i < len(*gnomes);  {
			var a = Move_gnome(board,mask_board,gnomes,i);
			i+=a
		}
		
		sort.Slice(*gnomes,func(i, j int) bool {
			if (*gnomes)[i].y == (*gnomes)[j].y {
				return (*gnomes)[i].x < (*gnomes)[j].x
			} else {
				return (*gnomes)[i].y < (*gnomes)[j].y
			}
		})

		printGnomes(*gnomes)
		if is_won(*gnomes){
			break
		}
		rnd ++
	}
	return result(*gnomes,rnd)
}


func Part1(fname string,expected int,elf_attack int ) int {

	var file, err = os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}

	var board []string

	defer file.Close()
	var scanner = bufio.NewScanner(file)
	for scanner.Scan() {
		board = append(board, scanner.Text())
	}

	var board_matr [][]byte = make([][]byte, len(board))
	var mask_board_matr [][]int = make([][]int, len(board))
	for  i := 0; i<len(board); i++ {
		board_matr[i] = []byte(board[i])
		mask_board_matr [i] = make([]int,len(board[i]))
	}

	if err = scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var gnomes = Parse_gnomes(board_matr,elf_attack)
	var elves_before = count_elves(gnomes)
	
	var res = Round(board_matr,mask_board_matr,&gnomes)

	var elves_after = count_elves(gnomes)
	fmt.Printf("Result %d for %s expected %d \n",res,fname,expected)
	if elves_before > elves_after{
		res = -res
	} 
	return res
}

func Part2(fname string,upper int ) int {
	var res = Part1(fname,0,3)
	for i:= 4; i< upper; i++{
		var res_now = Part1(fname,0,i)
		if res < 0 && res_now > 0{
			fmt.Printf("Won with attack %d %d\n", i,Part1(fname,0,i+1))
			return i
		}
		res = res_now
	}
	return 1
}

func main() {
	// Part1("input");
	// Part1("input_test");
	// Part1("inp_combat");
	// Part1("inp_real_combat",27730,15);
	// Part1("inp_real_combat2",36334,3);
	// Part1("inp_real_combat3",39514,3);
	// Part1("inp_real_combat4",27755,3);
	// Part1("inp_real_combat5",28944,3);
	// Part1("inp_real_combat6",18740,3);
	Part1("input",190012,3);

	// Part1("inp_real_combat",27730,15);
	// Part2("inp_real_combat",30);
	// Part2("input",50);
	// Part1("inp22",30,3);
}
