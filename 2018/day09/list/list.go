package list

import (
	"fmt"
)

type Node struct {
	prev int
	next int
}
type MapList struct {
	list map[int]Node
}

func (self *MapList) Add(num int, start int, offset int) {
	if self.list == nil {
		self.list = make(map[int]Node)
		self.list[num] = Node{num, num}
	} else {
        start = self.Offset(start,offset)

		var next = self.list[start].next
		var prev = start

		var next_upd = self.list[next]
		next_upd.prev = num
		self.list[next] = next_upd

		var prev_upd = self.list[prev]
		prev_upd.next = num
		self.list[prev] = prev_upd

		self.list[num] = Node{prev: prev, next: next}
	}
}

func (self *MapList) Remove(num int) int {
	if self.list == nil {
		self.list = make(map[int]Node)
		return -1
	} else {
		if val, in := self.list[num]; in {
			var next = val.next
			var prev = val.prev

			var next_upd = self.list[next]
			next_upd.prev = prev
			self.list[next] = next_upd

			var prev_upd = self.list[prev]
			prev_upd.next = next
			self.list[prev] = prev_upd

			delete(self.list, num)

			return num
		} else {
			return -1
		}
	}
}

func (self *MapList) Offset(num int, offset int) int {
    if offset < 0 {
        for ;offset != 0; offset++{
            num = self.list[num].prev
        }
    } else {
        for ;offset > 0; offset-- {
            num = self.list[num].next
        }
    }
	return num
}

func (self *MapList) Print(start int,insert_key int) {
    if self.list != nil {
        fmt.Printf("%d ", start)
        for crnt := self.list[start].next;crnt!=start; {
            if insert_key == crnt{
                fmt.Printf("(%d) ", crnt)
            } else {
                fmt.Printf("%d ", crnt)
            }
            crnt = self.list[crnt].next
        }
        fmt.Println()
    }
}
