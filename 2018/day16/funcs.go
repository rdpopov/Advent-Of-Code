package main

func addr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] + reg[B]
	return res
}
func addi (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] + B
	return res
}
func mulr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] * reg[B]
	return res
}
func muli (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] * B
	return res
}
func banr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] & reg[B]
	return res
}
func bani (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] & B
	return res
}
func borr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] | reg[B]
	return res
}
func bori (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] | B
	return res
}
func setr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	// var B = inp[2]
	var C = inp[3]
	res[C] = reg[A] 
	return res
}
func seti (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	// var B = inp[2]
	var C = inp[3]
	res[C] = A
	return res
}
func gtir (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(A > reg[B])
	return res
}

func gtri (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(reg[A] > B)
	return res
}
func gtrr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(reg[A] > reg[B])
	return res
}

func eqir (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(A == reg[B])
	return res
}
func eqri (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(reg[A] == B)
	return res
}
func eqrr (inp []int, reg []int) (res []int) {
	res = make([]int, 4)
	copy(res,reg)
	// var op = inp[0]
	var A = inp[1]
	var B = inp[2]
	var C = inp[3]
	res[C] = b2i(reg[A] == reg[B])
	return res
}
