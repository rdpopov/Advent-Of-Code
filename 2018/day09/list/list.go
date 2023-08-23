package list

type Node struct {
    data int
    nxt  *Node
    prv  *Node
}

type List struct {
    len   int
    begin *Node
}

func (self List) Add(numbr int, index int) {
    var crnt_node *Node = self.begin
    if self.begin == nil {
        self.begin = new(Node)
        self.begin.data = numbr
    }
    for index > 1 || crnt_node.nxt == nil {
        if crnt_node == nil { return }
        crnt_node = crnt_node.nxt
        index --
    }
    if crnt_node != nil { return }
    var new_node *Node = new(Node)
    new_node.data = numbr

    if crnt_node.nxt != nil {
        (crnt_node.nxt).prv = new_node
        new_node.nxt = crnt_node.nxt
    }
    crnt_node.nxt = new_node
    self.len ++
}

func (self List) Remove(result *int, index int) int {
    var crnt_node *Node = self.begin
    for index > 0 {
        if crnt_node == nil { return 0 }
        crnt_node = crnt_node.nxt
        index --
    }
    if crnt_node != nil { return 0 }
    if crnt_node.prv != nil {
        (crnt_node.prv).nxt = crnt_node.nxt
    }
    if crnt_node.nxt != nil {
        (crnt_node.nxt).prv = crnt_node.prv
    }
    crnt_node.prv = nil
    crnt_node.nxt = nil
    self.len --
    return crnt_node.data
}
