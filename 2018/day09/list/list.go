package list

type Node struct {
    prev  int
    next  int
}
type MapList struct {
    list map[int]Node
    head int
}

func (self *MapList) Add(num int){
    if self.list == nil {
        self.list = make(map[int]Node)
        self.list[num] = Node{num, num}
        self.head = num
    } else {
        var next = self.list[self.head].prev
        var next_upd = self.list[next]
        next_upd.prev = num
        var prev_upd = self.list[prev]

    }
}
func (self *MapList) Remove(num int){}

func (self *MapList) Offset(num int,offset int) int {
    return 0
}
func (self *MapList) Print(num int){}

func (self *MapList) SetHead(num int){}
func (self *MapList) MoveHead(num int){}

